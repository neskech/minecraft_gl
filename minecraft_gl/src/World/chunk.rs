use std::collections::HashMap;

use crate::Renderer::worldRenderer::Vertex;

use super::{block::{Block, BlockRegistry}, State};
use nalgebra as na;

//TODO HAVE Z REPRESENT THE HEIGHT. IN THE ACUTAL GAME WORLD, JUST CALL THE y COORDINATE Z and BE DONE WITH IT
pub const CHUNK_BOUNDS_X: u32 = 8;
pub const CHUNK_BOUNDS_Y: u32 = 8;
pub const CHUNK_BOUNDS_Z: u32 = 16;
//placeholder
pub struct Chunk{
    pub Blocks: Vec<Block>,
    pub Mesh: Vec<Vertex>,
    pub DynamicState: HashMap<u32, HashMap<String, State>>,
    pub StaticState: HashMap<u32, HashMap<String, State>>,

    pub ChunkPosition: (u32, u32)
}

impl Chunk{
    pub fn New() -> Self {
        let mut blocks = Vec::new();
        blocks.reserve((CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z) as usize);

        Self {
            Blocks: blocks,
            Mesh: Vec::new(), //can reserve?
            DynamicState: HashMap::new(),
            StaticState: HashMap::new(),
            ChunkPosition: (0, 0) //TODO feed this in as a parameter
        }
    }

    pub fn OfHeight(heightLevel: u32) -> Self {
        let mut blocks = Vec::new();
        let size = (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z) as usize;
        blocks.reserve(size);
        for i in 0..size {
            blocks.push(Block { ID: 3 } );
        }
        
        let offset =  CHUNK_BOUNDS_X *  heightLevel * CHUNK_BOUNDS_Y;
        blocks.iter_mut().skip(offset as usize).for_each(|b| *b = Block { ID : 1 });

        Self {
            Blocks: blocks,
            Mesh: Vec::new(), //can reserve?
            DynamicState: HashMap::new(),
            StaticState: HashMap::new(),
            ChunkPosition: (0, 0) //TODO feed this in as a parameter
        }
    }

    pub fn GetBlockStateAt(&mut self, coordinate: (u32, u32, u32)) -> Option<&mut HashMap<String, State>> {
        let idx = coordinate.0 + CHUNK_BOUNDS_X * (coordinate.2 + coordinate.1 * CHUNK_BOUNDS_Y);
        if self.DynamicState.contains_key(&idx) {
            return Some(self.DynamicState.get_mut(&idx).unwrap());
        }
        else if self.StaticState.contains_key(&idx) {
            return Some(self.StaticState.get_mut(&idx).unwrap());
        }
        else {
            None
        }
    }

    pub fn DestroyBlock(&mut self, coordinate: (u32, u32, u32)){
        //TODO make an onDestroy() function in blockBehaviors and call it here. It should take in the block state as a parameter before you delete it
        //Remove the block from the blocks array and the state
        let idx = To1D(coordinate);
        self.Blocks[idx as usize] = Block::Air();
        if self.DynamicState.contains_key(&idx) {
            self.DynamicState.remove(&idx);
        }
        else if self.StaticState.contains_key(&idx) {
            self.StaticState.remove(&idx);
        }
    }

    pub fn EmplaceBlock(&mut self, coordinate: (u32, u32, u32), block: &Block) -> bool{
        //call blockBehaviors[idx].onPlace() -> Option<Hashmap> and add that hashmap to block states
        //bool for success
        false
    }

    pub fn PropogateBlockUpdate(&mut self, origin: (u32, u32, u32)){

    }

    pub fn Clear(&mut self){
        self.Blocks.clear();
        self.Mesh.clear();
    }

    pub fn GenerateBlocks(&mut self){

    }

    pub fn GenerateMesh(&mut self, blockRegistry: &BlockRegistry){

        let directions = [na::Vector3::new(1i32, 0i32, 0i32), na::Vector3::new(-1i32, 0i32, 0i32), 
                                                                   na::Vector3::new(0i32, 1i32, 0i32), na::Vector3::new(0i32, -1i32, 0i32),
                                                                   na::Vector3::new(0i32, 0i32, 1i32), na::Vector3::new(0i32, 0i32, -1i32)];
        //TODO contain logic for drawing faces on chunk boundraries
        //TODO if the chunk on the bounds is unloaded, don't draw the faces. Else, query that other chunk for a present block
        let airID = blockRegistry.IDofBlock("Air");

        for x in 0..CHUNK_BOUNDS_X {
            for y in 0..CHUNK_BOUNDS_Y {
                for z in 0..CHUNK_BOUNDS_Z {

                    let mut i: u8 = 0;
                    let currIDX = To1DVec(na::Vector3::new(x as i32, y as i32, z as i32));
                    let currBlock = &self.Blocks[currIDX as usize];
                    if currBlock.ID == airID {
                        continue;
                    }
                    for direc in directions {
                        
                        let new3D = na::Vector3::new(x as i32, y as i32, z as i32) + direc;

              

                        if new3D.x >= 0 && new3D.x < CHUNK_BOUNDS_X as i32 && new3D.y >= 0 && new3D.y < CHUNK_BOUNDS_Y as i32 &&  new3D.z >= 0 && new3D.z < CHUNK_BOUNDS_Z as i32 {

                            let idx = To1DVec(new3D);
 
                            let block = &self.Blocks[idx as usize];
                            if block.ID != airID { i += 1; continue;}
                        }

                     

                        let offset = na::Vector3::new(0.5f32, 0.5f32, 0.5f32) + 0.5f32 * na::Vector3::new(direc.x as f32, direc.y as f32, direc.z as f32);
                        let intOffset = na::Vector3::new(offset.x as i32, offset.y as i32, offset.z as i32);
                        let axisA = na::Vector3::new(direc.y, direc.z, direc.x);
                        let axisB = axisA.cross(&direc).abs();
                        
                        println!("Block at {}, {}, {}", x, y, z);
                        let off = [0, 1];
                        for a in 0..2 {
                            for b in 0..2 {
                                let pos = axisA.abs() * off[a] + axisB * off[b] + na::Vector3::new(x as i32, y as i32, z as i32) + intOffset;
                                println!("POS!! {:?} offset direc {:?} axis A {:?} axis B {:?}", pos, direc, axisA, axisB);
                                let mut texID = 0;
                                if let Some(data) = &blockRegistry.GetAttributesOf(&currBlock).TextureData {
                                    texID = data.TextureID + data.Offsets[i as usize];
                                }
                                self.Mesh.push(Vertex { pos: [pos.x as f32, pos.y as f32, pos.z as f32],
                                texID: texID, 
                            quadID: (a * 2 + b) as u32});
                                //self.Mesh.push(Vertex { Data: ( (pos.x & 0b1111) | (pos.y & 0b1111) << 4 | (pos.z & 0b11111111) << 8 |  (texID as i32 & 0b11111111) << 16 | ((a * 2 + b) as i32 & 0b11) << 24 | (i as i32 & 0b111) >> 26 ) as u32 });
                                //println!("Pos from bits {}, {}", self.Mesh[self.Mesh.len() - 1].Data & 0b1111, self.Mesh[self.Mesh.len() - 1].Data >> 4 & 0b1111)
                                //TODO Build the vertex data
                                //TODO provide the texture ID using the face index, position data using 1D chunk cords, and face index for lighting
                            }
                        }
                    
                        i += 1;
                   }

                }

                
            }

          
            //look in all 6 directions, only building faces if an air block is present
        }
    
        
    }

}

fn To3D(idx: u32) -> (u32, u32, u32){
    (idx % CHUNK_BOUNDS_X, idx / (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z), idx % (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) / CHUNK_BOUNDS_X)
}

fn To1D(cord: (u32, u32, u32)) -> u32{
    cord.0 + CHUNK_BOUNDS_X * (cord.1 + cord.2 * CHUNK_BOUNDS_Y)
}

fn To3DVec(idx: u32) -> na::Vector3<u32>{
    na::Vector3::new(idx % CHUNK_BOUNDS_X, idx / (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z), idx % (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) / CHUNK_BOUNDS_X)
}

fn To1DVec(cord: na::Vector3<i32>) -> i32{
    cord.x + CHUNK_BOUNDS_X as i32 * (cord.y + cord.z * CHUNK_BOUNDS_Y as i32)
}

fn ToVec<T>(cord: (T, T, T)) -> na::Vector3<T>{
    na::Vector3::new(cord.0, cord.1, cord.2)
}