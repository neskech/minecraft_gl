use std::{collections::{HashSet, HashMap, VecDeque}, sync::{Arc, Mutex}, hash::Hash, thread};
use nalgebra as na;
use bracket_noise::prelude::FastNoise;
use rand::Rng;

use crate::{Scene::camera::Camera, World::biomeGenerator::Biome, Renderer::renderer};

use super::{block::BlockRegistry, chunk::{Chunk, CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z, CHUNK_BOUNDS_Y}, item::ItemRegistry, crafting::CraftingRegistry, biomeGenerator::{BiomeGenerator, NoiseParameters}, ReadBiomeGenerators};


const DEFAULT_RENDER_DISTANCE: i32 = 1;
const MAX_CHUNK_GENERATION_PER_FRAME: usize = 1;
const MAX_RENDER_DISTANCE: i32= 10;

const CHUNK_BIOME_DISTANCE_THRESHOLD: f32 = 0.2f32;

const CHUNK_BATCH_SIZE: usize = 1;

//TODO May be an issue where a chunk is added to the pipeline multiple times. Say if the user moved to fast, and as a chunk was in the
//TODO remesh list it got added to the regeneration list. An issue?
//TODO CHANGE chunk pos to i32's


pub struct World{
    pub Chunks: HashMap<na::Vector2<i32>, Chunk>,

    RenderDistance: i32,
    TargetPosition: (i32, i32),

    pub RenderList: HashSet<*const Chunk>,

    //creation queue acts as an intermediary between the chunks dictionary and the other thread creating chunks
    //this is so we don't have to lock the chunks dictionary, as it would have to be locked for 100% of the time as its being sent to the renderer
    CreationQueue: Arc<Mutex<VecDeque<(na::Vector2<i32>, Chunk)>>>,
    RemovalQueue: VecDeque<na::Vector2<i32>>,

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

            CreationQueue: Arc::new(Mutex::new(VecDeque::new())),
            RemovalQueue: VecDeque::new(),

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
    //     if self.RemovalQueue.len() > 0 {
    //          println!("Breaking with {} size", self.RemovalQueue.len());
    //     }

    //     let l = self.CreationQueue.lock().unwrap().len();
    //     if l > 0 {
    //         println!("Creation with {} size", l);
    //    }


        while self.RemovalQueue.len() > 0 {
            let vec = self.RemovalQueue.front().unwrap().clone();
            let mut exists = false;

            {
                 if let Some(chunk) = self.Chunks.get(&vec) {
                      exists = true;
                      self.RenderList.remove(&(chunk as *const Chunk));
                      self.RemovalQueue.pop_front().unwrap();
                 }
            }

            if exists {
                self.Chunks.remove(&vec).unwrap();
            } else {
                break;
            }
    
        }

        let mut len = self.CreationQueue.lock().unwrap().len();
        while len > 0 {
            let el = self.CreationQueue.lock().unwrap().pop_front().unwrap();
            self.Chunks.insert(el.0, el.1);
            len = self.CreationQueue.lock().unwrap().len();
        }

        let currChunkPos = ToChunkPos(targetPos);
        if self.TargetPosition != currChunkPos{
            println!("Swap!");
            //TODO prevent (1, 1) and (-1, -1) and (1, -1) (-1, 1) swaps
            let direction = (currChunkPos.0 - self.TargetPosition.0, currChunkPos.1 - self.TargetPosition.1);
            if direction.0 != 0 && direction.1 != 0 {
                self.TranslateChunks(self.TargetPosition, currChunkPos, (direction.0, 0));
                self.TranslateChunks(self.TargetPosition, currChunkPos, (0, direction.1));
            } else {
                self.TranslateChunks(self.TargetPosition, currChunkPos, direction);
            }
        }
        self.TargetPosition = currChunkPos;
        self.RenderListUpdate();
        
   
    }

    fn generateChunk(&mut self, chunkPositions: Vec<na::Vector2<i32>>){

        let blockReg = self.BlockRegistry.clone();
        let biomeGens = self.BiomeGenerators.clone();
        let queque = self.CreationQueue.clone();

        let mut adj = HashMap::with_capacity(chunkPositions.len());
        let d = [-1, 1_i32];
        for vec in &chunkPositions {
            let mut arr: [Option<usize>; 4] = [None; 4];

            //[(--1, 0), (1, 9), (0, -1), (0, 1)]
            for a in 0..2 {
                let v = na::Vector2::new(d[a] + vec.x, vec.y);
                arr[a] = if self.Chunks.contains_key(&v) {Some(&self.Chunks[&v] as *const _ as usize)} else {None};

                let v = na::Vector2::new(vec.x, d[a] + vec.y);
                arr[a +  2] = if self.Chunks.contains_key(&v) {Some(&self.Chunks[&v] as *const _ as usize)} else {None};
            }

            adj.insert(*vec, arr);
        }


        const THROTTLE: u128 = (1000_u128 / 60_u128) / MAX_CHUNK_GENERATION_PER_FRAME as u128; //in milliseconds

        rayon::spawn(move || {

            //TODO add a queue for this guy to work through. Each time this method is called a new thread is dispatched
            //TODO the queue is to just add on work so that only a single thread is going through all of them
            //TODO to have a single thread, only spawn a new thread if the queue is empty

            let mut regenMap = HashMap::with_capacity(chunkPositions.len());
            for pos in &chunkPositions {
                let start = std::time::Instant::now();

                let mut chunk = Chunk::New((pos.x, pos.y), 0f32);
                //generate the blocks...
                chunk.GenerateBlocks(biomeGens.lock().unwrap().get_mut(&Biome::Forest).unwrap());
                regenMap.insert(*pos, chunk);

                if start.elapsed().as_millis() < THROTTLE {
                    thread::sleep(std::time::Duration::from_millis((THROTTLE - start.elapsed().as_millis()) as u64));
                }

            }

            //TODO can omit the use of so many hashmaps and can instead 
            //TODO have a vector of these chunks where the index correspond to the chunk posistions array
            let d = [-1, 1_i32];
            for vec in &chunkPositions{
    
                //[(--1, 0), (1, 9), (0, -1), (0, 1)]
                let arr = adj.get_mut(vec).unwrap();
                for a in 0..2 {
                    if arr[a].is_none() {
                        let v = na::Vector2::new(d[a] + vec.x, vec.y);
                        arr[a] = if regenMap.contains_key(&v) {Some(&regenMap[&v] as *const _ as usize)} else {None};
                    }
                    
                    if arr[a + 2].is_none() {
                        let v = na::Vector2::new(vec.x, d[a] + vec.y);
                        arr[a +  2] = if regenMap.contains_key(&v) {Some(&regenMap[&v] as *const _ as usize)} else {None};
                    }
                }

               // println!("chunk {:?} with adj {:?} {}", vec, adj[idx], regenMap.contains_key(&na::Vector2::new(-1 ,-1)));
            }

            for (pos, mut chunk) in regenMap.into_iter() {
                let start = std::time::Instant::now();

                let mut arr = [None; 4];
                
                let a = adj[&pos];
                for i in 0..4 {
                    if let Some(u) = a[i] {
                         arr[i] = Some(u as *const Chunk);
                    } else {
                        arr[i] = None;
                    }
                }


                //println!("{:?}", arr);

                chunk.GreedyMesh(&arr, &*blockReg);
                //then add to the chunks dictionary...
                queque.lock().unwrap().push_front((pos.clone(), chunk));

                 if start.elapsed().as_millis() < THROTTLE {
                    thread::sleep(std::time::Duration::from_millis((THROTTLE - start.elapsed().as_millis()) as u64));
                }
            }
       
        });
        println!("done on main thread!");

    }

    fn TranslateChunks(&mut self, oldPos: (i32, i32), newPos: (i32, i32), direc: (i32, i32)){
        //TODO need to remesh chunks adjacent to new chunks
      
        //calculate the stuff that needs to be removed and the stuff that needs to be added
        //make a vec of positions to add and parallel iterate over that
        let size = (self.RenderDistance * 2 + 1) as i32;
        let rd = self.RenderDistance as i32;

        let rangeFunc = |d: i32, c: i32, sign: i32| -> (i32, i32) {
            let rt = if d == 0 { (-rd + c, rd + c) } else { (rd * sign * d + c, rd * sign * d + c) };
            rt
        };

        let removeRangeX =  rangeFunc(direc.0, oldPos.0, -1);
        let removeRangeY = rangeFunc(direc.1, oldPos.1, -1);
        for a in maybe_reverse_range(removeRangeX.0, removeRangeX.1) {
            for b in maybe_reverse_range(removeRangeY.0, removeRangeY.1) {
                self.RemovalQueue.push_front(na::Vector2::new(a, b));
            }
        }

        //invert the ranges for the adding
        let addRangeX = rangeFunc(direc.0, newPos.0, 1);
        let addRangeY = rangeFunc(direc.1, newPos.1, 1);

        let mut vec = Vec::with_capacity(size as usize);
        for a in maybe_reverse_range(addRangeX.0, addRangeX.1) {
            for b in maybe_reverse_range(addRangeY.0, addRangeY.1) {
                vec.push(na::Vector2::new(a, b));
            }
        }

        self.generateChunk(vec);

    }

    pub fn RenderListUpdate(&mut self){
        for a in -self.RenderDistance..=self.RenderDistance {
            for b in -self.RenderDistance..=self.RenderDistance {
                let pos = na::Vector2::new(a + self.TargetPosition.0, b + self.TargetPosition.1);
                if let Some(chunk) = self.Chunks.get(&pos){
                    self.RenderList.insert(chunk as *const Chunk);
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
                        newChunks.push(pos);
                     } else {
                        self.Chunks.remove(&pos);
                     }
                }

            }
        }
        
        self.RenderDistance = renderDistance;

        // //everytime chunks internal buffer is reallocated (given a new capacity) the pointers in the render list get invalidated. Do this to be wary of that
        // self.RenderList.clear();
        // self.Chunks.reserve(((self.RenderDistance * 2 + 1) * (self.RenderDistance * 2 + 1)) as usize);
        self.generateChunk(newChunks);

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