use std::collections::HashSet;

use crate::Scene::camera::Camera;

use super::{block::BlockRegistry, chunk::{Chunk, CHUNK_BOUNDS_X, CHUNK_BOUNDS_Y, CHUNK_BOUNDS_Z}, item::ItemRegistry, crafting::CraftingRegistry};


const DEFAULT_RENDER_DISTANCE: usize = 1;
const MAX_CHUNK_GENERATION_PER_FRAME: usize = 3;
const MAX_CHUNK_REMESH_PER_FRAME: usize = 3;

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

    BlockRegistry: BlockRegistry,
    ItemRegistry: ItemRegistry,
    CraftingRegistry: CraftingRegistry,

}

impl World{
    pub fn New(craftingRegistry: CraftingRegistry, blockRegistry: BlockRegistry, itemRegistry: ItemRegistry) -> Self{
        let mut chunks: Vec<Chunk> = Vec::new();
        chunks.reserve(DEFAULT_RENDER_DISTANCE * DEFAULT_RENDER_DISTANCE);
        chunks.push(Chunk::OfHeight(2, (0, 0)));
        chunks[0].GenerateMesh(&blockRegistry);
        
        let mut renderList = HashSet::new();
        renderList.insert(0);

        Self{
            Chunks: chunks,

            RenderDistance: DEFAULT_RENDER_DISTANCE,
            TargetPosition: (0f32, 0f32),

            RegenerationList: Vec::new(),
            RemeshList: Vec::new(),
            CandidateList: Vec::new(),
            RenderList: renderList,

            BlockRegistry: blockRegistry,
            ItemRegistry: itemRegistry,
            CraftingRegistry: craftingRegistry,
        }
    }

    pub fn Update(&mut self, targetPos: (f32, f32), camera: &Camera){
        let prevChunkPos = ToChunkPos(self.TargetPosition);
        let currChunkPos = ToChunkPos(targetPos);
        if prevChunkPos != currChunkPos{
            self.SwapChunks((currChunkPos.0 - prevChunkPos.0, currChunkPos.1 - prevChunkPos.1));
        }
        self.TargetPosition = targetPos;
    }

    pub fn SwapChunks(&mut self, direction: (i32, i32)){
       
        let iterator = if direction.0 < 0 || direction.1 < 0 {0..self.Chunks.len() - 1} else {self.Chunks.len()-1..1};
        let sign = if direction.0 < 0 || direction.1 < 0 {-1i32} else {1i32};

        let i = if direction.0 < 0 || direction.1 < 0 {self.Chunks.len() - 1} else {0};
        let mut hold: Chunk = self.Chunks[
            if direction.0 == 0 {
               i % self.Chunks.len() * self.RenderDistance as usize + i / self.Chunks.len()
            }
            else {
                i / self.Chunks.len() * self.RenderDistance as usize + i % self.Chunks.len()
            }
        ].clone();


        for i in iterator{
            let currIdx: usize;
            let nextIdx: usize;
            //y direction
            if direction.0 == 0 {
                currIdx = i % self.Chunks.len() * self.RenderDistance as usize + i / self.Chunks.len();
                nextIdx = (i as i32 + 1 * sign) as usize % self.Chunks.len() * self.RenderDistance as usize + (i as i32 + 1 * sign) as usize / self.Chunks.len();
            }
            //x direction
            else {
                currIdx = i / self.Chunks.len() * self.RenderDistance as usize + i % self.Chunks.len();
                nextIdx = (i as i32 + 1 * sign) as usize / self.Chunks.len() * self.RenderDistance as usize + (i as i32 + 1 * sign) as usize % self.Chunks.len();
            }

            //check if the current index is a deletion chunk...
            let deletionChunk = if direction.0 == 0 {
                //y direction
                direction.1 < 0 && i % self.Chunks.len() == self.RenderDistance - 1 || direction.1 > 0 && i % self.Chunks.len() == 0
            }
            else {
                //x direction
                direction.0 < 0 && i % self.Chunks.len() == self.RenderDistance - 1 || direction.0 > 0 && i % self.Chunks.len() == 0
            };

            if deletionChunk {
                hold = self.Chunks[currIdx].to_owned();
                //TODO see if this copies it
                //TODO We want to see if these are shallow copies and not full copies, test it
                //TODO by printing the addresses
                self.Chunks[currIdx].Clear();
                self.RegenerationList.push(currIdx);
            }
            let temp = self.Chunks[nextIdx].clone();
            self.Chunks[nextIdx] = hold;
            hold = temp;
        }
    }

    pub fn RegenerationUpdate(&mut self){
        let mut count = 0;
        for idx in self.RegenerationList.len()-1..0 {
            if count >= MAX_CHUNK_GENERATION_PER_FRAME {
                return;
            }

            let index = self.RegenerationList[idx];
            self.Chunks[index].GenerateBlocks();
            self.RemeshList.push(index);
            self.RegenerationList.remove(idx);
            count += 1;
        }
    }

    pub fn RemeshUpdate(&mut self){
        let mut count = 0;
        for idx in self.RemeshList.len()-1..0 {
            if count >= MAX_CHUNK_REMESH_PER_FRAME {
                return;
            }

            let index = self.RemeshList[idx];

            let row: i32 = index as i32 / self.RenderDistance as i32;
            let col: i32 = index as i32 % self.RenderDistance as i32;
            //If <a,0> idx = -1 -> 0, 1 -> 1
            //If <0, b> idx = 2 + -1 -> 0, 1 -> 1
            let chunkList = [
                if col - 1 == -1 {None} else {Some(&self.Chunks[(row * self.RenderDistance as i32 + col - 1) as usize])},//Left
                if col + 1 == self.RenderDistance as i32 {None} else {Some(&self.Chunks[(row * self.RenderDistance as i32 + col + 1) as usize])}, //Right
                if row - 1 == -1 as i32 {None} else {Some(&self.Chunks[((row - 1)* self.RenderDistance as i32 + col) as usize])}, //Right
                if row + 1 == self.RenderDistance as i32 {None} else {Some(&self.Chunks[((row + 1) * self.RenderDistance as i32 + col) as usize])} //Right
            ];
            self.Chunks.get_mut(index).unwrap().GenerateMesh(&self.BlockRegistry);
            self.CandidateList.push(index);
            self.RemeshList.remove(idx);
            count += 1;
        }
    }

    pub fn CandidateUpdate(&mut self, camera: &Camera){
        for idx in self.CandidateList.len()-1..0 {

            let index = self.CandidateList[idx];
            let center = nalgebra::Vector3::new(
                ((index as u32 % CHUNK_BOUNDS_X) * CHUNK_BOUNDS_X) as f32 + CHUNK_BOUNDS_X as f32 / 2f32,
                ((index as u32 % CHUNK_BOUNDS_X) * CHUNK_BOUNDS_X) as f32 + CHUNK_BOUNDS_X as f32 / 2f32,
                CHUNK_BOUNDS_Z as f32 / 2f32
            );
            if camera.Fustrum.CheckChunk(&center){
                self.RenderList.insert(index);
            }
            else {
                self.RenderList.remove(&index);
            }
        }
    }

    pub fn UpdateRenderDistance(&mut self, renderDistance: u32){
        let numNewChunks = renderDistance * renderDistance - self.RenderDistance as u32 * self.RenderDistance as u32;

    }
    
}



fn ToChunkPos(pos: (f32, f32)) -> (i32, i32){
    (( ( pos.0 - if pos.0 < 0f32 {CHUNK_BOUNDS_X as f32} else {0f32} ) / CHUNK_BOUNDS_X as f32) as i32, 
    ( ( pos.1 - if pos.1 < 0f32 {CHUNK_BOUNDS_Y as f32} else {0f32} ) / CHUNK_BOUNDS_Y as f32) as i32)
}