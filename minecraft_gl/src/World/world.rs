use nalgebra as na;
use bracket_noise::prelude::FastNoise;
use rand::Rng;
use std::sync::mpsc;

use std::{collections::{HashSet, HashMap, VecDeque}, 
          sync::{Arc, Mutex, mpsc::Receiver}, thread
         };

use crate::{World::{block::BlockRegistry, 
            chunk::{Chunk, CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z}, 
            item::ItemRegistry, crafting::CraftingRegistry, 
            biomeGenerator::{BiomeGenerator, Biome, NoiseParameters}, 
            ReadBiomeGenerators
            }, Scene::camera::Camera
           };


const DEFAULT_RENDER_DISTANCE: usize = 1;
const MAX_RENDER_DISTANCE: usize = 10;

pub struct World{
    pub Chunks: HashMap<na::Vector2<i32>, Arc<Chunk>>,
    pub RenderList: HashSet<*const Chunk>, //TODO change to basic list

    //Used for syncing the workers
    WorkerQueue: VecDeque<Vec<(bool, Chunk)>>,
    RemovalQueue: VecDeque<na::Vector2<i32>>,
    Reciever: Option<Receiver<Arc<Chunk>>>,
    IsWorking: Arc<Mutex<bool>>,

    BlockRegistry: Arc<BlockRegistry>,
    ItemRegistry: ItemRegistry, //to be used
    CraftingRegistry: CraftingRegistry, //to be used

    BiomeGenerators: Arc<Mutex<HashMap<Biome, Box<dyn BiomeGenerator + Send>>>>,
    BiomeNoise: NoiseParameters, //to be used
    BiomeNoiseGenerator: FastNoise, //to be used

    RenderDistance: usize,
    TargetPosition: (i32, i32),
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
                panic!("Error! World construction failed due to failure to read 
                        biome generators. The error:\n{}", msg.to_string())
            }
        };

   
        let mut self_ = Self{
            Chunks: HashMap::with_capacity( (DEFAULT_RENDER_DISTANCE * 2 + 1) * 
                                            (DEFAULT_RENDER_DISTANCE * 2 + 1)),
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

            RenderDistance: 0,
            TargetPosition: (0i32, 0i32),
        };

        self_.RenderDistanceUpdate(DEFAULT_RENDER_DISTANCE);
        self_
    }

    pub fn Update(&mut self, targetPos: (f32, f32), _: &Camera){
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
            self.TranslateChunks(self.TargetPosition, currChunkPos);
        }
        self.TargetPosition = currChunkPos;
        self.RenderListUpdate();
        
   
    }

    fn generationUpdate(&mut self) {
        //first check if the current thread is still working
        let b = self.IsWorking.lock().unwrap().to_owned();
        if b || self.WorkerQueue.len() == 0 {
            return;
        }

        //then make a new vector of adjacents to send to the thread
        let work = self.WorkerQueue.pop_front().unwrap();
        let mut buffer: Vec<(Chunk, bool, [Option<Arc<Chunk>>; 4])> = 
                    Vec::with_capacity(work.len());

        //add relevant adjacent chunks to the buffer
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

        //spawn the thread
        let (tx, rx) = mpsc::channel();
        self.Reciever = Some(rx);

        let biomeGens = self.BiomeGenerators.clone();
        let blockReg = self.BlockRegistry.clone();
        let isWorking = self.IsWorking.clone();
        *isWorking.lock().unwrap() = true;

        thread::spawn(move || {
            //iterate through our buffer, generating chunk blocks
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

            //add any new adjacencies from the chunks we just created
            //TODO find a way to optomize this because copying all these chunks is bad ):
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

            //now mesh the chunks
            for (mut chunk, _, adj) in buffer.into_iter() {
                chunk.GreedyMesh(&adj, &blockReg);
                tx.send(Arc::new(chunk)).unwrap();
            }

            //set the bool to false or send a message to signify its all over
            *isWorking.lock().unwrap() = false;

        });
    }

    fn TranslateChunks(&mut self, oldPos: (i32, i32), newPos: (i32, i32)){
        let radius: i32 = self.RenderDistance.try_into().unwrap();

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

        //TODO just send a message to the thread to quit its execution
        //TODO do this later. This is an optimization. It is not neccesary

    }

    pub fn RenderListUpdate(&mut self){
        let extents: i32 = self.RenderDistance.try_into().unwrap();

        for a in -extents..=extents {
            for b in -extents..=extents {
                let pos = na::Vector2::new(a + self.TargetPosition.0, b + self.TargetPosition.1);

                if let Some(chunk) = self.Chunks.get(&pos){
                    self.RenderList.insert(chunk.as_ref() as *const Chunk);
                }
     
            }
        }

    }


    pub fn RenderDistanceUpdate(&mut self, renderDistance: usize){
        if renderDistance >= MAX_RENDER_DISTANCE {
            eprintln!("Error! Cannot change render distance to {} 
                      since it is {} above the max render distance of {}!",
                      renderDistance, renderDistance - MAX_RENDER_DISTANCE,
                      MAX_RENDER_DISTANCE);
            return;
        }
        else if renderDistance == self.RenderDistance {
            return;
        }

        let increase = renderDistance > self.RenderDistance;

        let target = self.TargetPosition;
        let mut newChunks = Vec::with_capacity({
            let sq = (renderDistance * 2 + 1) * (renderDistance * 2 + 1);
            let sq2 = (self.RenderDistance * 2 + 1) * (self.RenderDistance * 2 + 1);
            let rem = sq - sq2;
            rem.max(0)
        });

        let extents: i32 = self.RenderDistance.try_into().unwrap();
        for a in -extents..=extents {
            for b in -extents..=extents {

                let pos = na::Vector2::new(a + target.0, b + target.1);

                if self.RenderDistance == 0 || 
                   (pos.x < -extents || pos.x > extents) && 
                   (pos.y < -extents || pos.y > extents) 
                {
                     if increase {
                        let chunk = Chunk::New((pos.x, pos.y), 0.0f32);
                        newChunks.push((false, chunk));
                     } 
                     else {
                        self.Chunks.remove(&pos);
                     }
                }
                else if pos.x.abs() == extents && pos.y.abs() == extents{
                    //Remesh all chunks on the ORIGINAL outer ring
                    //TODO remesh the inner chunks
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