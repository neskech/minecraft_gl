use std::collections::HashMap;
use crate::Renderer::worldRenderer::Vertex;
use super::{block::{Block, BlockRegistry, TextureData}, State, biomeGenerator::{Biome, BiomeGenerator}};
use nalgebra as na;

//TODO GET THE MATH WORKING OUT BETTER
//TODO HAVE Z REPRESENT THE HEIGHT. IN THE ACUTAL GAME WORLD, JUST CALL THE y COORDINATE Z and BE DONE WITH IT
pub const CHUNK_BOUNDS_X: u32 = 15;
pub const CHUNK_BOUNDS_Y: u32 = 60;
pub const CHUNK_BOUNDS_Z: u32 = 15;
pub const TOTAL_CHUNK_SIZE: u32 = CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z;

#[derive(Clone)]
pub struct Chunk{
    pub Blocks: Vec<Block>,
    pub Mesh: Vec<Vertex>,
    pub DynamicState: HashMap<u32, HashMap<String, State>>,
    pub StaticState: HashMap<u32, HashMap<String, State>>,
    pub Position: (i32, i32),

    pub Biome: Biome,
    pub BiomeValue: f32,
}

impl Chunk{
    pub fn New(chunkPos: (i32, i32), biomeValue: f32) -> Self {
        Self {
            Blocks: Vec::with_capacity(TOTAL_CHUNK_SIZE as usize),
            //approcimation of surface area
            Mesh: Vec::with_capacity(f32::powf(TOTAL_CHUNK_SIZE as f32, 2f32 / 3f32) as usize * 6), 
            DynamicState: HashMap::new(),
            StaticState: HashMap::new(),
            Position: chunkPos,

            Biome: Biome::None, 
            BiomeValue: biomeValue
        }
    }

    pub fn OfHeight(heightLevel: u32, chunkPos: (i32, i32)) -> Self {
        let mut blocks = Vec::with_capacity(TOTAL_CHUNK_SIZE as usize);
        let offset = CHUNK_BOUNDS_X * (heightLevel) * CHUNK_BOUNDS_Z;
        for i in 0..TOTAL_CHUNK_SIZE {
            if i < offset {
                blocks.push(Block { ID: 2 } );
            }
            else {
               blocks.push(Block { ID: 0 } );
            }
        }

        Self {
            Blocks: blocks,
            Mesh: Vec::new(), //can reserve?
            DynamicState: HashMap::new(),
            StaticState: HashMap::new(),
            Position: chunkPos,

            Biome: Biome::None, 
            BiomeValue: -1f32
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

    pub fn EmplaceBlock(&mut self, _coordinate: (u32, u32, u32), _block: &Block) -> bool{
        //call blockBehaviors[idx].onPlace() -> Option<Hashmap> and add that hashmap to block states
        //bool for success
        false
    }

    pub fn PropogateBlockUpdate(&mut self, _origin: (u32, u32, u32)){

    }

    pub fn Clear(&mut self){
        self.Blocks.clear();
        self.Mesh.clear();
    }

    pub fn ClearMesh(&mut self){
        self.Mesh.clear();
    }

    pub fn GenerateBlocks(&mut self, generator: &mut Box<dyn BiomeGenerator + Send>){

        //TODO maybe change surface ampltidue in json file to max height and when making heightmap do
        //TODO Surface level + (max_height - surface level) * noise_normalized
        self.Blocks.resize(TOTAL_CHUNK_SIZE as usize, Block::Air());
        generator.Generate(&mut self.Blocks, self.Position.0, self.Position.1);
    }

    pub fn GenerateMesh(&mut self, adjacentChunks: &[Option<*const Chunk>; 4], blockRegistry: &BlockRegistry, enableAdjacencyCulling: bool){
        let directions = [na::Vector3::new(1i32, 0i32, 0i32), na::Vector3::new(-1i32, 0i32, 0i32), 
                                                                   na::Vector3::new(0i32, 1i32, 0i32), na::Vector3::new(0i32, -1i32, 0i32),
                                                                   na::Vector3::new(0i32, 0i32, 1i32), na::Vector3::new(0i32, 0i32, -1i32)];
        let mut temp: Vec<Vertex> = Vec::new();
        //println!("Chunks!! {:?}", adjacentChunks);                                                   
        //Loop over each axis of the chunk
        for x in 0..CHUNK_BOUNDS_X {
            for y in 0..CHUNK_BOUNDS_Y {
                for z in 0..CHUNK_BOUNDS_Z {
    
                    let mut faceID: u8 = 0;
                    
                    //push the mutable reference out of scope after this. That's why this is in a scope
                    let currBlock = self.Blocks[To1DVec(na::Vector3::new(x as i32, y as i32, z as i32)) as usize].clone();
    
                    //If the current block is air, then there is nothing to draw. Continue...
                    if currBlock.ID == 0 {
                        continue;
                    }
                    //Each block has 6 faces in 6 different directions. Loop over each direction to build each face
                    for direc in directions {
                        //The 3D coordinate of a block in the direction of the current vector in this loop
                        let new3D = na::Vector3::new(x as i32, y as i32, z as i32) + direc;
                        //check for out of bounds on each axis
                        let outX = new3D.x < 0 || new3D.x >= CHUNK_BOUNDS_X as i32;
                        let outY = new3D.y < 0 || new3D.y >= CHUNK_BOUNDS_Y as i32;
                        let outZ = new3D.z < 0 || new3D.z >= CHUNK_BOUNDS_Z as i32;
    
                        //grab the appropiate block based on these variables
                        let mut adjacentBlock: Block = Block::Air();
                        if outX {
                            //the x index we use to sample from the adjacent chunk
                            if new3D.x < 0 {
                                let x = CHUNK_BOUNDS_X - 1;
                                if let Some(val) = adjacentChunks[0] {
                                        adjacentBlock = unsafe { (*val).Blocks[To1D((x, new3D.y as u32, new3D.z as u32)) as usize].clone() };
                                }
                                else if enableAdjacencyCulling {faceID += 1; continue}
                            } else {
                                let x = 0;
                                if let Some(val) = adjacentChunks[1] {
                                     adjacentBlock =  unsafe { (*val).Blocks[To1D((x, new3D.y as u32, new3D.z as u32)) as usize].clone() };
                                    
                                }
                                else if enableAdjacencyCulling {faceID += 1; continue}
                            }
    
                        }
                        else if outZ {
                            //the y index we use to sample from the adjacent chunk
                            if new3D.z < 0 {
                                let z = CHUNK_BOUNDS_Z - 1;
                                if let Some(val) = adjacentChunks[2] {
                                     adjacentBlock = unsafe { (*val).Blocks[To1D((new3D.x as u32, new3D.y as u32, z)) as usize].clone() };
                                }
                                else if enableAdjacencyCulling {faceID += 1; continue}
                            } else {
                                let z = 0;
                                if let Some(val) = adjacentChunks[3] {
                                      adjacentBlock = unsafe { (*val).Blocks[To1D((new3D.x as u32, new3D.y as u32, z)) as usize].clone() };
                                }
                                else if enableAdjacencyCulling {faceID += 1; continue}
                            }
                        }
                        else if !outY { //Means y axis is not out of bounds
                           
                            adjacentBlock = self.Blocks[To1DVec(new3D) as usize].clone();
                            
                        }
    
                        if !outY && adjacentBlock.ID != 0 && !blockRegistry.GetAttributesOf(&adjacentBlock).Decor{
                            faceID += 1;
                            continue;
                        }
    
                        //Check if it is a decor block
                        if blockRegistry.GetAttributesOf(&currBlock).Decor && y < CHUNK_BOUNDS_Y - 1 && x < CHUNK_BOUNDS_X - 1 && z < CHUNK_BOUNDS_Z - 1{ //TODO bounds check
    
                            let mut texID = 0;
                            if let Some(TextureData::Decoration(data)) = &blockRegistry.GetAttributesOf(&currBlock).TextureData {
                                    texID = data.TextureID;
                            }
    
                            //Curr, +1 y, +1 y +1 x +1 z, +1x +1 z
                            let offsets = [(0, 0, 0), (1, 0, 1), (0, 1, 0), (1, 1, 1)]; //TL, TR, BL, BR
                            for i in 0..4 {
                                let newPos = (x + offsets[i].0, y + offsets[i].1, z + offsets[i].2);
                                let data =  newPos.0  | newPos.2 << 4 | newPos.1 << 8 |  texID << 16 | (i as u32) << 24 | (faceID as u32) << 26;
                                temp.push(Vertex { Data: data } );
                            }
    
                            let offsets = [(0, 0, 1), (1, 0, 0), (0, 1, 1), (1, 1, 0),]; //TL, TR, BL, BR
                            for i in 0..4 {
                                let newPos = (x  + offsets[i].0, y  + offsets[i].1, z  + offsets[i].2);
                                let data =  (newPos.0 | newPos.2 << 4 | newPos.1 << 8 |  texID << 16 | (i as u32) << 24 | (0 as u32) << 26) as u32;
                                temp.push(Vertex { Data: data } );
                            }
                            continue;
                        }
    
    
                        let offset = na::Vector3::new(0.5f32, 0.5f32, 0.5f32) + 0.5f32 * na::Vector3::new(direc.x as f32, direc.y as f32, direc.z as f32);
                        let intOffset = na::Vector3::new(offset.x as i32, offset.y as i32, offset.z as i32);
                        let axisA = na::Vector3::new(direc.y, direc.z, direc.x);
                        let axisB = axisA.cross(&direc);
     
                        
                        let off = [0, 1];
                        for a in 0..2 {
                            for b in 0..2 {
                                let pos = axisA.abs() * off[a] + axisB.abs() * off[b] + na::Vector3::new(x as i32, y as i32, z as i32) + intOffset;
                
                                let mut texID = 0;
                                //TODO textureData should not be an optional. Either its real or the null texture
                                if let Some(TextureData::SixSided(data)) = &blockRegistry.GetAttributesOf(&currBlock).TextureData {
                                    texID = data.TextureID + data.Offsets[faceID as usize];
                                }
        
    
                                let mut id = (a * 2 + b) as u32;
                                if direc.x == 1 || direc.x == -1 {
                                    id = (b * 2 + a) as u32;
                                }
                                let dat = ( pos.x  | pos.z << 4 | pos.y << 8 |  (texID as i32) << 16 | (id as i32) << 24 | (faceID as i32) << 26 ) as u32;
                                self.Mesh.push(Vertex { Data: dat } );
                               // println!("FACE ID {} and bits {:08b} and real {}", dat >> 24 & 0x7, dat >> 24 & 0x7, faceID);
                            }
                        }
                    
                        faceID += 1;
                   }
    
                }
    
                
            }
    
            }
            //look in all 6 directions, only building faces if an air block is present
            for item in &temp {
                self.Mesh.push(*item);
            }
    
    }
}

pub fn GenerateMesh(chunks: &mut Vec<Chunk>, idx: usize, adjacentChunks: &[Option<usize>; 4], blockRegistry: &BlockRegistry, enableAdjacencyCulling: bool){
    let directions = [na::Vector3::new(1i32, 0i32, 0i32), na::Vector3::new(-1i32, 0i32, 0i32), 
                                                               na::Vector3::new(0i32, 1i32, 0i32), na::Vector3::new(0i32, -1i32, 0i32),
                                                               na::Vector3::new(0i32, 0i32, 1i32), na::Vector3::new(0i32, 0i32, -1i32)];
    let mut temp: Vec<Vertex> = Vec::new();
    //println!("Chunks!! {:?}", adjacentChunks);                                                   
    //Loop over each axis of the chunk
    for x in 0..CHUNK_BOUNDS_X {
        for y in 0..CHUNK_BOUNDS_Y {
            for z in 0..CHUNK_BOUNDS_Z {

                let mut faceID: u8 = 0;
                
                //push the mutable reference out of scope after this. That's why this is in a scope
                let currBlock = chunks[idx].Blocks[To1DVec(na::Vector3::new(x as i32, y as i32, z as i32)) as usize].clone();

                //If the current block is air, then there is nothing to draw. Continue...
                if currBlock.ID == 0 {
                    continue;
                }
                //Each block has 6 faces in 6 different directions. Loop over each direction to build each face
                for direc in directions {
                    //The 3D coordinate of a block in the direction of the current vector in this loop
                    let new3D = na::Vector3::new(x as i32, y as i32, z as i32) + direc;
                    //check for out of bounds on each axis
                    let outX = new3D.x < 0 || new3D.x >= CHUNK_BOUNDS_X as i32;
                    let outY = new3D.y < 0 || new3D.y >= CHUNK_BOUNDS_Y as i32;
                    let outZ = new3D.z < 0 || new3D.z >= CHUNK_BOUNDS_Z as i32;

                    //grab the appropiate block based on these variables
                    let mut adjacentBlock: Block = Block::Air();
                    if outX {
                        //the x index we use to sample from the adjacent chunk
                        if new3D.x < 0 {
                            let x = CHUNK_BOUNDS_X - 1;
                            if let Some(val) = adjacentChunks[0] {
                                    adjacentBlock = chunks[val].Blocks[To1D((x, new3D.y as u32, new3D.z as u32)) as usize].clone();
                            }
                            else if enableAdjacencyCulling {faceID += 1; continue}
                        } else {
                            let x = 0;
                            if let Some(val) = adjacentChunks[1] {
                                 adjacentBlock = chunks[val].Blocks[To1D((x, new3D.y as u32, new3D.z as u32)) as usize].clone();
                                
                            }
                            else if enableAdjacencyCulling {faceID += 1; continue}
                        }

                    }
                    else if outZ {
                        //the y index we use to sample from the adjacent chunk
                        if new3D.z < 0 {
                            let z = CHUNK_BOUNDS_Z - 1;
                            if let Some(val) = adjacentChunks[2] {
                                 adjacentBlock = chunks[val].Blocks[To1D((new3D.x as u32, new3D.y as u32, z)) as usize].clone();
                            }
                            else if enableAdjacencyCulling {faceID += 1; continue}
                        } else {
                            let z = 0;
                            if let Some(val) = adjacentChunks[3] {
                                  adjacentBlock = chunks[val].Blocks[To1D((new3D.x as u32, new3D.y as u32, z)) as usize].clone();
                            }
                            else if enableAdjacencyCulling {faceID += 1; continue}
                        }
                    }
                    else if !outY { //Means y axis is not out of bounds
                       
                        adjacentBlock = chunks[idx].Blocks[To1DVec(new3D) as usize].clone();
                        
                    }

                    if !outY && adjacentBlock.ID != 0 && !blockRegistry.GetAttributesOf(&adjacentBlock).Decor{
                        faceID += 1;
                        continue;
                    }

                    //Check if it is a decor block
                    let currChunk = &mut chunks[idx];
                    if blockRegistry.GetAttributesOf(&currBlock).Decor && y < CHUNK_BOUNDS_Y - 1 && x < CHUNK_BOUNDS_X - 1 && z < CHUNK_BOUNDS_Z - 1{ //TODO bounds check

                        let mut texID = 0;
                        if let Some(TextureData::Decoration(data)) = &blockRegistry.GetAttributesOf(&currBlock).TextureData {
                                texID = data.TextureID;
                        }

                        //Curr, +1 y, +1 y +1 x +1 z, +1x +1 z
                        let offsets = [(0, 0, 0), (1, 0, 1), (0, 1, 0), (1, 1, 1)]; //TL, TR, BL, BR
                        for i in 0..4 {
                            let newPos = (x + offsets[i].0, y + offsets[i].1, z + offsets[i].2);
                            let data =  newPos.0  | newPos.2 << 4 | newPos.1 << 8 |  texID << 16 | (i as u32) << 24 | (faceID as u32) << 26;
                            temp.push(Vertex { Data: data } );
                        }

                        let offsets = [(0, 0, 1), (1, 0, 0), (0, 1, 1), (1, 1, 0),]; //TL, TR, BL, BR
                        for i in 0..4 {
                            let newPos = (x  + offsets[i].0, y  + offsets[i].1, z  + offsets[i].2);
                            let data =  (newPos.0 | newPos.2 << 4 | newPos.1 << 8 |  texID << 16 | (i as u32) << 24 | (0 as u32) << 26) as u32;
                            temp.push(Vertex { Data: data } );
                        }
                        continue;
                    }


                    let offset = na::Vector3::new(0.5f32, 0.5f32, 0.5f32) + 0.5f32 * na::Vector3::new(direc.x as f32, direc.y as f32, direc.z as f32);
                    let intOffset = na::Vector3::new(offset.x as i32, offset.y as i32, offset.z as i32);
                    let axisA = na::Vector3::new(direc.y, direc.z, direc.x);
                    let axisB = axisA.cross(&direc);
 
                    
                    let off = [0, 1];
                    for a in 0..2 {
                        for b in 0..2 {
                            let pos = axisA.abs() * off[a] + axisB.abs() * off[b] + na::Vector3::new(x as i32, y as i32, z as i32) + intOffset;
            
                            let mut texID = 0;
                            //TODO textureData should not be an optional. Either its real or the null texture
                            if let Some(TextureData::SixSided(data)) = &blockRegistry.GetAttributesOf(&currBlock).TextureData {
                                texID = data.TextureID + data.Offsets[faceID as usize];
                            }
    

                            let mut id = (a * 2 + b) as u32;
                            if direc.x == 1 || direc.x == -1 {
                                id = (b * 2 + a) as u32;
                            }
                            let dat = ( pos.x  | pos.z << 4 | pos.y << 8 |  (texID as i32) << 16 | (id as i32) << 24 | (faceID as i32) << 26 ) as u32;
                            currChunk.Mesh.push(Vertex { Data: dat } );
                           // println!("FACE ID {} and bits {:08b} and real {}", dat >> 24 & 0x7, dat >> 24 & 0x7, faceID);
                        }
                    }
                
                    faceID += 1;
               }

            }

            
        }

        }
        //look in all 6 directions, only building faces if an air block is present
        for item in &temp {
            chunks[idx].Mesh.push(*item);
        }

}

pub fn To3D(idx: u32) -> (u32, u32, u32){
    (idx % CHUNK_BOUNDS_X, idx / (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z), idx % (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) / CHUNK_BOUNDS_X)
}

pub fn To1D(cord: (u32, u32, u32)) -> u32{
    cord.0 + CHUNK_BOUNDS_X * (cord.2 + cord.1 * CHUNK_BOUNDS_Z)
}

fn To1DUsize(cord: (usize, usize, usize)) -> usize{
    cord.0 + CHUNK_BOUNDS_X as usize * (cord.2 + cord.1 * CHUNK_BOUNDS_Z as usize)
}

fn To3DVec(idx: u32) -> na::Vector3<u32>{
    na::Vector3::new(idx % CHUNK_BOUNDS_X, idx / (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z), idx % (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) / CHUNK_BOUNDS_X)
}

fn To1DVec(cord: na::Vector3<i32>) -> i32{
    cord.x + CHUNK_BOUNDS_X as i32 * (cord.z + cord.y * CHUNK_BOUNDS_Z as i32)
}

fn ToVec<T>(cord: (T, T, T)) -> na::Vector3<T>{
    na::Vector3::new(cord.0, cord.1, cord.2)
}