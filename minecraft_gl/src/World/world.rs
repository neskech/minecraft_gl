use std::collections::{HashSet, HashMap};

use noise::{Seedable, NoiseFn};
use rand::Rng;

use crate::{Scene::camera::Camera, World::biomeGenerator::Biome};

use super::{block::BlockRegistry, chunk::{Chunk, CHUNK_BOUNDS_X, CHUNK_BOUNDS_Z, GenerateMesh}, item::ItemRegistry, crafting::CraftingRegistry, biomeGenerator::{BiomeGenerator, NoiseParameters}, ReadBiomeGenerators};


const DEFAULT_RENDER_DISTANCE: usize = 2;
const MAX_CHUNK_GENERATION_PER_FRAME: usize = 3;
const MAX_CHUNK_REMESH_PER_FRAME: usize = 3;
const MAX_RENDER_DISTANCE: usize = 10;

const CHUNK_BIOME_DISTANCE_THRESHOLD: f32 = 0.2f32;

//TODO May be an issue where a chunk is added to the pipeline multiple times. Say if the user moved to fast, and as a chunk was in the
//TODO remesh list it got added to the regeneration list. An issue?
//TODO CHANGE chunk pos to i32's
pub struct World{
    pub Chunks: Vec<Chunk>,

    RenderDistance: usize,
    TargetPosition: (f32, f32),

    RegenerationList: Vec<usize>,
    RemeshList: Vec<usize>,
    CandidateList: Vec<usize>,
    pub RenderList: HashSet<usize>,
    FinishedRegneration: bool,

    BlockRegistry: BlockRegistry,
    ItemRegistry: ItemRegistry,
    CraftingRegistry: CraftingRegistry,

    BiomeGenerators: HashMap<Biome, Box<dyn BiomeGenerator>>,
    BiomeNoise: NoiseParameters,

}

impl World{
    pub fn New(craftingRegistry: CraftingRegistry, blockRegistry: BlockRegistry, itemRegistry: ItemRegistry) -> Self{
        let mut chunks: Vec<Chunk> = Vec::new();
        chunks.reserve(DEFAULT_RENDER_DISTANCE * DEFAULT_RENDER_DISTANCE);

        let mut rng = rand::thread_rng();
        let noise =  NoiseParameters {
            Octaves: 6,
            Seed: rng.gen_range(0..10000),
            Frequency: 1f64,
            Lacunarity: std::f64::consts::PI * 2.0 / 3.0,
            Persistance: 0.5f64,
        };
        
        let mut w = noise::Fbm::new();
        w = w.set_seed(noise.Seed);
        noise.ApplyToFBM(&mut w);
        chunks.push(Chunk::New((0,0), w.get([0f64, 0f64]) as f32));

        let renderList = HashSet::new();
        let mut regen = Vec::new();
        regen.push(((DEFAULT_RENDER_DISTANCE * 2 + 1) * (DEFAULT_RENDER_DISTANCE * 2 + 1)) / 2);

        let map = match ReadBiomeGenerators(&blockRegistry) {
            Ok(val) => val,
            Err(msg) => {
                panic!("Error! World construction failed due to failure to read biome generators. The error:\n{}", msg.to_string())
            }
        };

        let mut s = Self{
            Chunks: chunks,

            RenderDistance: 0,
            TargetPosition: (0f32, 0f32),

            RegenerationList: regen,
            RemeshList: Vec::new(),
            CandidateList: Vec::new(),
            RenderList: renderList,
            FinishedRegneration: false,

            BlockRegistry: blockRegistry,
            ItemRegistry: itemRegistry,
            CraftingRegistry: craftingRegistry,

            BiomeGenerators: map,
            BiomeNoise: noise,
        };

        s.RenderDistanceUpdateFunc(DEFAULT_RENDER_DISTANCE as u32);
        let mut i = 0;
        for chunk in &s.Chunks {
            println!("Chunk idx {} and pos {:?}", i, chunk.Position);
            i += 1;
        }
        s.RegenerationList.sort();
   
        s
    }

    pub fn Update(&mut self, targetPos: (f32, f32), camera: &Camera){
        let prevChunkPos = ToChunkPos(self.TargetPosition);
        let currChunkPos = ToChunkPos(targetPos);
        if prevChunkPos != currChunkPos{
            self.SwapChunks((currChunkPos.0 - prevChunkPos.0, currChunkPos.1 - prevChunkPos.1));
        }
        self.TargetPosition = targetPos;
        self.RegenerationUpdate();
        self.RemeshUpdate();
        self.CandidateUpdate(camera);
    }

    pub fn SwapChunks(&mut self, direction: (i32, i32)){
        //TODO is inclusive correct?
        let iterator: Vec<usize> = if direction.0 < 0 || direction.1 > 0 {(0..self.Chunks.len()).collect()} else {(0..self.Chunks.len()).rev().collect()};
        let sign = if direction.0 < 0 || direction.1 > 0 {1i32} else {-1i32};

        let size: usize = self.RenderDistance * 2 + 1;
        let i = if direction.0 < 0 || direction.1 < 0 {self.Chunks.len() - 1} else {0};
        let mut hold: Chunk = self.Chunks[
            if direction.0 == 0 {
               i % size * size+ i / size
            }
            else {
                i / size * size + i % size
            }
        ].clone();

        println!("SWAP TIME BABY!! {:?} chunks {}", direction, self.Chunks.len());
        let mut i = 0;
        for chunk in &self.Chunks {
            println!("Chunk idx {} and pos {:?}", i, chunk.Position);
            i += 1;
        }

        let mut w = noise::Fbm::new();
        w = w.set_seed(self.BiomeNoise.Seed);
        self.BiomeNoise.ApplyToFBM(&mut w);
        //TODO Make sure to update the chunk positions as well
        for i in iterator.into_iter(){
            let currIdx: usize;
            let nextIdx: usize;
            //y direction
            if direction.0 == 0 {
                currIdx = i % size * size + i / size;
                nextIdx = (i as i32 + 1 * sign) as usize % size * size as usize + (i as i32 + 1 * sign) as usize / size;
            }
            //x direction
            else {
                currIdx = i;
                nextIdx = (i as i32 + 1 * sign) as usize;
            }

            //check if the current index is a deletion chunk...
            let deletionChunk = if direction.0 == 0 {
                //y direction
                //if y < 0 and row == last row or if y > 0 and row == first row
                direction.1 > 0 && i % size == 0 || direction.1 < 0 && i % size == size - 1
            }
            else {
                //x direction
                //if x < 0 and col == last col or if x > 0 and col == first col
                direction.0 > 0 && i % size == size - 1 || direction.0 < 0 && i % size == 0
            };

            let skip = if direction.0 == 0 {
                //y direction
                //if y < 0 and row == last row or if y > 0 and row == first row
                direction.1 > 0 && i % size == size - 1 || direction.1 < 0 && i % size == 0
            }
            else {
                //x direction
                //if x < 0 and col == last col or if x > 0 and col == first col
                direction.0 > 0 && i % size == 0 || direction.0 < 0 && i % size == size - 1
            };

            //TODO Send the chunks adjacent to the deletion chunks to be remeshed due to adjacent chunk cull facing
            println!("idx {} with curr idx {}", i, currIdx);

            if skip {
                continue;
            }

            if deletionChunk {
                println!("Deletion chunk at {}", i);
                hold = self.Chunks[currIdx].clone();
                //TODO see if this copies it
                //TODO We want to see if these are shallow copies and not full copies, test it
                //TODO by printing the addresses
                //TODO have chunk generateBlocks() call Clear() for you
                self.Chunks[currIdx].Clear();
                self.Chunks[currIdx].Position = (self.Chunks[currIdx].Position.0 + direction.0, self.Chunks[currIdx].Position.1 + direction.1);
                self.Chunks[currIdx].BiomeValue = w.get([self.Chunks[currIdx].Position.0 as f64, self.Chunks[currIdx].Position.1 as f64]) as f32;
                self.Chunks[currIdx].Biome = Biome::None;
                self.RegenerationList.push(currIdx);
            }
            let temp = self.Chunks[nextIdx].clone();
            self.Chunks[nextIdx] = hold; 
            self.Chunks[nextIdx].Position = (temp.Position.0 + direction.0, temp.Position.1 + direction.1);
            if deletionChunk {
                self.RemeshList.push(nextIdx);
            }
            hold = temp;
        }

        println!("AFTER SWAP BABY!!!1");
        i = 0;
        for chunk in &self.Chunks {
            println!("Chunk idx {} and pos {:?}", i, chunk.Position);
            i += 1;
        }
    }

    pub fn RegenerationUpdate(&mut self){
        self.FinishedRegneration = true; //will remain true if the vec is empty
        if self.RegenerationList.len() == 0 {return;}
        self.FinishedRegneration = false; //otherwise if it isn't empty, we still need to regenerate chunks

        let mut count = 0;
        for idx in (0..self.RegenerationList.len()).rev() {
            if count >= MAX_CHUNK_GENERATION_PER_FRAME {
                return;
            }

            let index = self.RegenerationList[idx];

            let row: i32 = index as i32 / (self.RenderDistance as i32 * 2 + 1);
            let col: i32 = index as i32 % (self.RenderDistance as i32 * 2 + 1);

            let chunkList = [
                if col - 1 == -1 {None} else {Some((row * (self.RenderDistance as i32 * 2 + 1) + col - 1) as usize)},//Left
                if col + 1 == self.RenderDistance as i32 * 2 + 1 {None} else {Some((row * (self.RenderDistance as i32 * 2 + 1) + col + 1) as usize)}, //Right
                if row + 1 == self.RenderDistance as i32 * 2 + 1 {None} else {Some(((row + 1) * (self.RenderDistance as i32 * 2 + 1) + col) as usize)}, //Right
                if row - 1 == -1 as i32 {None} else {Some(((row - 1) * (self.RenderDistance as i32 * 2 + 1) + col) as usize)}, //Right
            ];

            let mut minDifference = f32::MAX;
            let mut minBiome: Biome = Biome::None;
            for val in chunkList {
                if let Some(v) = val {
                    if self.Chunks[v].Biome == Biome::None {continue;}
                    let diff = f32::abs(self.Chunks[v].BiomeValue - self.Chunks[index].BiomeValue);
                    if diff < minDifference {
                        minDifference = diff;
                        minBiome = self.Chunks[v].Biome.clone();
                    }
                }
            }

            //check if the minimum distance is < threshold
            if minDifference <= CHUNK_BIOME_DISTANCE_THRESHOLD {
                print!("OOpsie dooppsie woopsy!!!");
                self.Chunks[index].Biome = minBiome;
            }
            else {
                print!("Else oppsie!!!");
                self.Chunks[index].Biome = Biome::Random();
            }

            let genBiome = self.Chunks[index].Biome.clone();
            println!("Generating blocks for {}", index);
            println!("Biom attr {:?} and the hashmap", genBiome);
            self.Chunks[index].GenerateBlocks(&self.BiomeGenerators[&genBiome]);
            self.RemeshList.push(index);
            self.RegenerationList.remove(idx);
            count += 1;
        }
    }

    //TODO only remesh after all chunks are done regenerating their blocks
    //TODO so that the face culling fully works
    pub fn RemeshUpdate(&mut self){
        if self.RemeshList.len() == 0 || !self.FinishedRegneration {return;} //only remesh chunks when generation is finished
        let mut count = 0;
        for idx in (0..self.RemeshList.len()).rev() {
            if count >= MAX_CHUNK_REMESH_PER_FRAME {
                return;
            }

            let index = self.RemeshList[idx];

            let row: i32 = index as i32 / (self.RenderDistance as i32 * 2 + 1);
            let col: i32 = index as i32 % (self.RenderDistance as i32 * 2 + 1);
            //If <a,0> idx = -1 -> 0, 1 -> 1
            //If <0, b> idx = 2 + -1 -> 0, 1 -> 1
                    
            let chunkList = [
                if col - 1 == -1 {None} else {Some((row * (self.RenderDistance as i32 * 2 + 1) + col - 1) as usize)},//Left
                if col + 1 == self.RenderDistance as i32 * 2 + 1 {None} else {Some((row * (self.RenderDistance as i32 * 2 + 1) + col + 1) as usize)}, //Right
                if row + 1 == self.RenderDistance as i32 * 2 + 1 {None} else {Some(((row + 1) * (self.RenderDistance as i32 * 2 + 1) + col) as usize)}, //Right
                if row - 1 == -1 as i32 {None} else {Some(((row - 1) * (self.RenderDistance as i32 * 2 + 1) + col) as usize)}, //Right
            ];
            println!("row {} col {}", row, col);
            GenerateMesh(&mut self.Chunks, index, &chunkList, &self.BlockRegistry, true);
            self.CandidateList.push(index);
            self.RemeshList.remove(idx);
            count += 1;
        }
    }

    pub fn CandidateUpdate(&mut self, camera: &Camera){
        if self.CandidateList.len() == 0 {return}
        for idx in (0..self.CandidateList.len()).rev() {

            let index = self.CandidateList[idx];
            let center = nalgebra::Vector3::new(
                ((index as u32 % CHUNK_BOUNDS_X) * CHUNK_BOUNDS_X) as f32 + CHUNK_BOUNDS_X as f32 / 2f32,
                ((index as u32 % CHUNK_BOUNDS_X) * CHUNK_BOUNDS_X) as f32 + CHUNK_BOUNDS_X as f32 / 2f32,
                CHUNK_BOUNDS_Z as f32 / 2f32
            );
            if camera.Fustrum.CheckChunk(&center) || true{
                self.RenderList.insert(index);
            }
            else {
                self.RenderList.remove(&index);
            }
        }
    }

    pub fn RenderDistanceUpdateFunc(&mut self, renderDistance: u32){
        if renderDistance as usize >= MAX_RENDER_DISTANCE {
            eprintln!("Error! Cannot change render distance to {} since it is {} above the max render distance of {}!",
            renderDistance, renderDistance - MAX_RENDER_DISTANCE as u32, MAX_RENDER_DISTANCE);
            return;
        }
        else if renderDistance as usize == self.RenderDistance {
            return;
        }

        let oldRD = self.RenderDistance;
        let sign: i32 = if self.RenderDistance as u32 > renderDistance {-1} else {1};
        let mut w = noise::Fbm::new();
        w = w.set_seed(self.BiomeNoise.Seed);
        self.BiomeNoise.ApplyToFBM(&mut w);

        while self.RenderDistance as u32 != renderDistance {
            println!("Going at it again!!");
            self.UpdateRenderDistance((self.RenderDistance as i32 + 1 * sign) as u32, &w);
            println!("Render dist {} chunks size {}", self.RenderDistance, self.Chunks.len());
        }

        let mut noDelete: HashSet<usize> = HashSet::new();
        println!("{} {}", oldRD, renderDistance);
        let shave = renderDistance as usize - oldRD;

        let mut offset = shave * (renderDistance as usize * 2 + 1) + shave;
        noDelete.insert(offset);
        for _ in 0..oldRD {
            for _ in 0..oldRD {
                offset += 1;
                noDelete.insert(offset);
            }
            offset += shave * 2;
        }

        for a in 0..self.Chunks.len() {
            if ! noDelete.contains(&a) {
                self.RegenerationList.push(a);
            }
        }
    }

    fn UpdateRenderDistance(&mut self, renderDistance: u32, noiseGenerator: &noise::Fbm){
    

        if renderDistance < self.RenderDistance as u32{
            let mut noDelete: HashSet<usize> = HashSet::new();
            let shave = self.RenderDistance - renderDistance as usize;

            let mut offset = shave * self.RenderDistance + shave;
            for _ in 0..renderDistance {
                for _ in 0..renderDistance {
                    noDelete.insert(offset);
                    offset += 1;
                }
                offset += shave * 2;
            }

            for a in self.Chunks.len()-1..=0 {
                if ! noDelete.contains(&a) {
                    self.Chunks.remove(a);
                }
            }
            self.RenderDistance = renderDistance as usize;
            return;
        }


        
        let numNewChunks = u32::pow(renderDistance * 2 + 1, 2) - u32::pow(self.RenderDistance as u32 * 2 + 1, 2);
        let arrayPos = ((renderDistance as i32 * 2 + 1) / 2,  (renderDistance as i32 * 2 + 1) / 2);
        let centerPos = self.Chunks[self.Chunks.len() / 2].Position;

        let mut set: Vec<(usize, Chunk)> = Vec::with_capacity(numNewChunks as usize);
        for i in 0..numNewChunks {
            //find the interval it resides in (top side, right side, bottom side, left side)
            let mut interval: u32 = 0;
            for a in (0..(renderDistance * 2) * 4).step_by((renderDistance * 2) as usize) {
                if i >= a && i <= a + renderDistance * 2 {
                    break;
                }
                interval += 1;
            }



            match interval {
                0 => { //top side
                    //as i increase, column must increase
                    let rowCol: (i32, i32) = (0, i as i32 - 0); //row column
                    let newPos = (rowCol.0 - arrayPos.0 + centerPos.0, rowCol.1 - arrayPos.1 + centerPos.1);
                    println!("0 RowCol {:?} idx {} newPos {:?} insertion index {}", rowCol, i, newPos, (rowCol.0 * (renderDistance * 2 + 1) as i32 + rowCol.1) as usize);
                    //Change <row, col> -> <x, y> where x = col, y = row. 
                    //In <row,col> going up is negative. Although, in window space up is negative also so no need to change it
                    let insertion = (rowCol.0 * (renderDistance * 2 + 1) as i32 + rowCol.1) as usize;
                    set.push((insertion, Chunk::New((newPos.1, -newPos.0), noiseGenerator.get([newPos.1 as f64, -newPos.0 as f64]) as f32)));
                },
                1 => { //right side
                    //as i increases, row must increase
                    let rowCol: (i32, i32) = (i as i32 - (renderDistance as i32 * 2), renderDistance as i32 * 2 + 1 - 1);
                    let newPos = (rowCol.0 - arrayPos.0 + centerPos.0, rowCol.1 - arrayPos.1 + centerPos.1);
                    println!("1 RowCol {:?} idx {} newPos {:?} insertion index {}", rowCol, i, newPos, (rowCol.0 * (renderDistance * 2 + 1) as i32 + rowCol.1) as usize);
                    let insertion = (rowCol.0 * (renderDistance * 2 + 1) as i32 + rowCol.1) as usize;
                    set.push((insertion, Chunk::New((newPos.1, -newPos.0), noiseGenerator.get([newPos.1 as f64, -newPos.0 as f64]) as f32)));
                },
                2 => { //bottom side
                    //As i increases, column must decrease
                    let rowCol: (i32, i32) = (renderDistance as i32 * 2 + 1 - 1, renderDistance as i32 * 2 + 1 - 1 - (i as i32 - (renderDistance as i32 * 2) * 2));
                    let newPos = (rowCol.0 - arrayPos.0 + centerPos.0, rowCol.1 - arrayPos.1 + centerPos.1);
                    println!("2 RowCol {:?} idx {} newPos {:?} insertion index {}", rowCol, i, newPos, (rowCol.0 * (renderDistance * 2 + 1)as i32 + rowCol.1) as usize);
                    let insertion = (rowCol.0 * (renderDistance * 2 + 1) as i32 + rowCol.1) as usize;
                    set.push((insertion, Chunk::New((newPos.1, -newPos.0), noiseGenerator.get([newPos.1 as f64, -newPos.0 as f64]) as f32)));
                },
                3 => { //left side
                    //As i increases, row must decrease
                    let rowCol: (i32, i32) = (renderDistance as i32 * 2 + 1 - 1 - (i as i32 - (renderDistance as i32 * 2) * 3), 0);
                    let newPos = (rowCol.0 - arrayPos.0 + centerPos.0, rowCol.1 - arrayPos.1 + centerPos.1);
                    println!("3 RowCol {:?} idx {} newPos {:?} insertion index {}", rowCol, i, newPos, (rowCol.0 * (renderDistance * 2 + 1)as i32 + rowCol.1) as usize);
                    let insertion = (rowCol.0 * (renderDistance * 2 + 1) as i32 + rowCol.1) as usize;
                    set.push((insertion, Chunk::New((newPos.1, -newPos.0), noiseGenerator.get([newPos.1 as f64, -newPos.0 as f64]) as f32)));
                },
                _ => {} //not possible
            }

        }

        set.sort_by(|x, y| x.0.cmp(&y.0)); 
        for el in &set{
            println!("hhehhe {}", el.0);
        } 
        for el in &set {
            //cloning the chunk shouldnt be that bad for performance since it holds no data as of yet
            self.Chunks.insert(el.0, el.1.clone());
            //TODO only thing changed was adding the index to the regeneration list
        }
        self.RenderDistance = renderDistance as usize;

    }
    
}


fn ToChunkPos(pos: (f32, f32)) -> (i32, i32){
    (( ( pos.0 - if pos.0 < 0f32 {CHUNK_BOUNDS_X as f32} else {0f32} ) / CHUNK_BOUNDS_X as f32) as i32, 
    ( ( pos.1 - if pos.1 < 0f32 {CHUNK_BOUNDS_Z as f32} else {0f32} ) / CHUNK_BOUNDS_Z as f32) as i32)
}