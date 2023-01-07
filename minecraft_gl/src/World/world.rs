use std::{collections::{HashSet, HashMap, VecDeque}, sync::{Arc, Mutex, mpsc::Receiver, atomic::AtomicBool}, hash::Hash, thread};
use nalgebra as na;
use bracket_noise::prelude::FastNoise;
use rand::Rng;
use std::sync::mpsc;
use std::sync::atomic;

use crate::{Scene::camera::Camera, World::{biomeGenerator::Biome, block}, Renderer::renderer};

use super::{block::BlockRegistry, chunk::{Chunk, CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z, CHUNK_BOUNDS_Y}, item::ItemRegistry, crafting::CraftingRegistry, biomeGenerator::{BiomeGenerator, NoiseParameters}, ReadBiomeGenerators};


const DEFAULT_RENDER_DISTANCE: i32 = 1;
const MAX_CHUNK_GENERATION_PER_FRAME: usize = 1;
const MAX_RENDER_DISTANCE: i32 = 10;

const CHUNK_BIOME_DISTANCE_THRESHOLD: f32 = 0.2f32;

const CHUNK_BATCH_SIZE: usize = 1;

//TODO May be an issue where a chunk is added to the pipeline multiple times. Say if the user moved to fast, and as a chunk was in the
//TODO remesh list it got added to the regeneration list. An issue?
//TODO CHANGE chunk pos to i32's


pub struct World{
    pub Chunks: HashMap<na::Vector2<i32>, Arc<Chunk>>,

    RenderDistance: i32,
    TargetPosition: (i32, i32),

    pub RenderList: HashSet<*const Chunk>, //TODO change to basic list


    //creation queue acts as an intermediary between the chunks dictionary and the other thread creating chunks
    //this is so we don't have to lock the chunks dictionary, as it would have to be locked for 100% of the time as its being sent to the renderer
    WorkerQueue: VecDeque<Vec<(bool, Chunk)>>,
    RemovalQueue: VecDeque<na::Vector2<i32>>,

    Reciever: Option<Receiver<Arc<Chunk>>>,

    IsWorking: Arc<Mutex<bool>>,


    BlockRegistry: Arc<BlockRegistry>,
    ItemRegistry: ItemRegistry,
    CraftingRegistry: CraftingRegistry,

    BiomeGenerators: Arc<Mutex<HashMap<Biome, Box<dyn BiomeGenerator + Send>>>>,
    BiomeNoise: NoiseParameters,
    BiomeNoiseGenerator: FastNoise,

}

impl World{
    pub fn New(craftingRegistry: CraftingRegistry, blockRegistry: BlockRegistry, itemRegistry: ItemRegistry) -> Self{


        let mut rng = rand::thread_rng();
        let noise =  NoiseParameters {
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 0.08f32,
            Lacunarity: (std::f64::consts::PI * 2.0 / 3.0) as f32,
            Persistance: 0.5f32,
        };
    
        let map = match ReadBiomeGenerators(&blockRegistry) {
            Ok(val) => val,
            Err(msg) => {
                panic!("Error! World construction failed due to failure to read biome generators. The error:\n{}", msg.to_string())
            }
        };

   
        let mut self_ = Self{
            Chunks: HashMap::with_capacity( ( (DEFAULT_RENDER_DISTANCE * 2 + 1) * (DEFAULT_RENDER_DISTANCE * 2 + 1) ) as usize),

            RenderDistance: 0,
            TargetPosition: (0i32, 0i32),

            RenderList: HashSet::new(),

            WorkerQueue: VecDeque::new(),
            RemovalQueue: VecDeque::new(),

            Reciever: None,

            IsWorking: Arc::new(Mutex::new(false)),

            BlockRegistry: Arc::new(blockRegistry),
            ItemRegistry: itemRegistry,
            CraftingRegistry: craftingRegistry,

            BiomeGenerators: Arc::new(Mutex::new(map)),
            BiomeNoise: noise,
            BiomeNoiseGenerator: FastNoise::new(),
        };

        self_.RenderDistanceUpdate(DEFAULT_RENDER_DISTANCE);
        self_
    }

    pub fn Update(&mut self, targetPos: (f32, f32), camera: &Camera){
        self.generationUpdate();

        while self.RemovalQueue.len() > 0 {
            let vec = self.RemovalQueue.front().unwrap().clone();
            let mut exists = false;

            
            if let Some(chunk) = self.Chunks.get(&vec) {
                exists = true;
                self.RenderList.remove(&(chunk.as_ref() as *const Chunk));
                self.RemovalQueue.pop_front().unwrap();
            }
            
            if exists {
                self.Chunks.remove(&vec).unwrap();
            }
    
        }

        if self.Reciever.is_some() {
            let recieve = self.Reciever.as_ref().unwrap().try_recv();
            match recieve {
                Ok(e) => {
                    let pos = e.Position;
                    let vec = na::Vector2::new(pos.0, pos.1);
                    self.Chunks.insert(vec, e);
                },
                _ => {}
            }
        }



        let currChunkPos = ToChunkPos(targetPos);
        if currChunkPos != self.TargetPosition {
            println!("{:?}, {:?}, {:?}", currChunkPos, self.TargetPosition, (self.TargetPosition.0 - currChunkPos.0, self.TargetPosition.1 - currChunkPos.1));
            self.TranslateChunks(self.TargetPosition, currChunkPos);
        }
        self.TargetPosition = currChunkPos;
        self.RenderListUpdate();
        
   
    }

    fn generationUpdate(&mut self) {
        //TODO first check if the current thread is still working
        let b = self.IsWorking.lock().unwrap().to_owned();
        if b || self.WorkerQueue.len() == 0 {
            return;
        }

        //TODO then make a new vector of adjacents to send to the thread
        let work = self.WorkerQueue.pop_front().unwrap();
        let mut buffer: Vec<(Chunk, bool, [Option<Arc<Chunk>>; 4])> = 
                    Vec::with_capacity(work.len());

        //TODO add relevant adjacent chunks to the buffer
        let d = [-1, 1];
        for (remesh, chunk) in work.into_iter() {
            let pos = chunk.Position;

            let mut adj = [None, None, None, None];
            for a in 0..2 {

                    let newPos = na::Vector2::new(pos.0 + d[a], pos.1);
                    if self.Chunks.contains_key(&newPos) {
                        adj[a] = Some(self.Chunks.get(&newPos).unwrap().clone());
                    }

                    let newPos = na::Vector2::new(pos.0, pos.1 + d[a]);
                    if self.Chunks.contains_key(&newPos) {
                        adj[a + 2] = Some(self.Chunks.get(&newPos).unwrap().clone());
                    }
                
            }

            buffer.push((chunk, remesh, adj));
        }

        //TODO spawn the thread
        let (tx, rx) = mpsc::channel();
        self.Reciever = Some(rx);

        let biomeGens = self.BiomeGenerators.clone();
        let blockReg = self.BlockRegistry.clone();
        let isWorking = self.IsWorking.clone();
        *isWorking.lock().unwrap() = true;

        thread::spawn(move || {
            //TODO iterate through our buffer, generating chunk blocks
            for (chunk, remesh, _) in &mut buffer {
                //TODO add throttling
                if ! *remesh {
                    chunk.GenerateBlocks(biomeGens
                                                   .lock()
                                                   .unwrap()
                                                   .get_mut(&Biome::Forest)
                                                   .unwrap()
                                        );
                }
            }

            //TODO add any new adjacencies from the chunks we just created
            let copy = buffer.clone();
            for (chunk_, _, adj) in &mut buffer {
                let pos = chunk_.Position;

                println!("{} {} {} {}", adj[0].is_some(), adj[1].is_some(), adj[2].is_some(), adj[3].is_some());
                for (chunk, _, _) in &copy {
                    let targetPos = chunk.Position;

                    for a in 0..2 {
                            let newPos = na::Vector2::new(pos.0 + d[a], pos.1);
                            if newPos.x == targetPos.0 && newPos.y == targetPos.1 {
                                adj[a] = Some(Arc::new(chunk.clone()));
                            }

                            let newPos = na::Vector2::new(pos.0, pos.1 + d[a]);
                            if newPos.x == targetPos.0 && newPos.y == targetPos.1 {
                                adj[a + 2] = Some(Arc::new(chunk.clone()));
                            }
                    }
                }
            }
            println!("HERE!!!");
            //TODO now mesh the chunks
            for (mut chunk, _, adj) in buffer.into_iter() {
                println!("{} {} {} {}", adj[0].is_some(), adj[1].is_some(), adj[2].is_some(), adj[3].is_some());
                chunk.GreedyMesh(&adj, &blockReg);
                tx.send(Arc::new(chunk));
            }

            //TODO set the atomic bool to false or send a message to signify its all over
            *isWorking.lock().unwrap() = false;

        });
    }

    fn TranslateChunks(&mut self, oldPos: (i32, i32), newPos: (i32, i32)){
        let radius = self.RenderDistance;

        let inRadius = |pos: (i32, i32), center: (i32, i32) | {
            return pos.0 >= center.0 - radius &&
                   pos.0 <= center.0 + radius &&
                   pos.1 >= center.1 - radius &&
                   pos.1 <= center.1 + radius;
        };

        let mut vec = Vec::new();

        for offY in -radius..=radius {
            for offX in -radius..=radius {

                let newPosOff = (newPos.0 + offX, newPos.1 + offY);
                //First, add chunks that are in the newPos area and NOT in the oldPos area
                if !inRadius(newPosOff, oldPos) {
                    //TODO add this chunk pos to the pipeline
                    let chunk = Chunk::New(newPosOff, 0.0f32);
                    vec.push((false, chunk));
                }

                let oldPosOff = (oldPos.0 + offX, oldPos.1 + offY);
                //Next, for chunks in the old radius and not in the new one, add to the removal queue
                if !inRadius(oldPosOff, newPos) {
                    self.RemovalQueue.push_back(na::Vector2::new(oldPosOff.0, oldPosOff.1));
                }

            }
        }

        //frontier chunks are adjacent to new chunks
        //we need to remesh all frontier chunks
        //We want these at the end of the buffer so we do another for loop
        //find a better way of doing this
        for offY in -radius..=radius {
            for offX in -radius..=radius {
                let newPosOff = (newPos.0 + offX, newPos.1 + offY);
                let oldPosOff = (oldPos.0 + offX, oldPos.1 + offY);

                if !inRadius(newPosOff, oldPos) {
                    continue;
                }
               
                let mut exit = false;
                let d = [-1_i32, 1];
                for a in 0..2 {
                    for b in 0..2 {
                        let offPos = (newPosOff.0 + d[a], newPosOff.1 + d[b]);
                        //check if this adjacent position is a NEW CHUNK
                        if !inRadius(offPos, oldPos) {
                            //if it is, remesh this little guy
                            let v = na::Vector2::new(newPosOff.0, newPosOff.1);
                            vec.push((true, self.Chunks.get(&v).unwrap().as_ref().clone()));

                            exit = true;
                            break;
                        }
                    }

                    if exit {break;}
                }
                if exit {break};

            }
        }

        self.WorkerQueue.push_back(vec);

        //Now, we there could still be some chunks we wish to remove in the pipeline
        //It's useless and a waste of computation to left them finish. so let us remove 
        //Those chunks from the generation and removal queues
        //TODO typedef vector2 to chunkpos
        let map : HashMap<na::Vector2<i32>, u32> = HashMap::new();
        //TODO just send a message to the thread to quit its execution

        //TODO do this later. This is an optimization. It is not neccesary

    }

    pub fn RenderListUpdate(&mut self){
        for a in -self.RenderDistance..=self.RenderDistance {
            for b in -self.RenderDistance..=self.RenderDistance {
                let pos = na::Vector2::new(a + self.TargetPosition.0, b + self.TargetPosition.1);


                if let Some(chunk) = self.Chunks.get(&pos){
                    self.RenderList.insert(chunk.as_ref() as *const Chunk);
                }
     
            }
        }

    }


    pub fn RenderDistanceUpdate(&mut self, renderDistance: i32){
        if renderDistance >= MAX_RENDER_DISTANCE {
            eprintln!("Error! Cannot change render distance to {} since it is {} above the max render distance of {}!",
            renderDistance, renderDistance - MAX_RENDER_DISTANCE, MAX_RENDER_DISTANCE);
            return;
        }
        else if renderDistance == self.RenderDistance {
            return;
        }

        let increase = renderDistance > self.RenderDistance;

        let target = self.TargetPosition;
        let mut newChunks = Vec::with_capacity(
            ( (renderDistance * 2 + 1) * (renderDistance * 2 + 1) - (self.RenderDistance * 2 + 1) * (self.RenderDistance * 2 + 1) ).max(0) as usize
        );

        for a in -renderDistance..=renderDistance {
            for b in -renderDistance..=renderDistance {
                let pos = na::Vector2::new(a + target.0, b + target.1);
                if self.RenderDistance == 0 || (pos.x < -self.RenderDistance || pos.x > self.RenderDistance) && (pos.y < -self.RenderDistance || pos.y > self.RenderDistance) {
                     if increase {
                        let chunk = Chunk::New((pos.x, pos.y), 0.0f32);
                        newChunks.push((false, chunk));

                        //add adjacent chunks

                     } else {
                        //TODO WTF?? use removal queue!!
                        self.Chunks.remove(&pos);
                     }
                }
                //The new chunks go on the outer ring of the chunks array
                //Remesh all chunks on the ORIGINAL outer ring
                else if pos.x.abs() == self.RenderDistance && pos.y.abs() == self.RenderDistance {
                    //WILL remesh this chunk ONLY once the outer ring is generated
                    //The only missing adjacent chunk is the chunk on that outer ring
                   // self.RemeshSet.insert(pos);
                }

            }
        }

        self.RenderDistance = renderDistance;
        self.WorkerQueue.push_back(newChunks);

    }

    


}

fn ToChunkPos(pos: (f32, f32)) -> (i32, i32){
    (( ( pos.0 - if pos.0 < 0f32 {CHUNK_BOUNDS_X as f32} else {0f32} ) / CHUNK_BOUNDS_X as f32) as i32, 
    ( ( pos.1 - if pos.1 < 0f32 {CHUNK_BOUNDS_Z as f32} else {0f32} ) / CHUNK_BOUNDS_Z as f32) as i32)
}

fn maybe_reverse_range(init: i32, end: i32) -> Box<dyn Iterator<Item=i32>> {
    if end < init {
        Box::new((end..=init).rev())
    } else {
        Box::new((init..=end))
    }
}