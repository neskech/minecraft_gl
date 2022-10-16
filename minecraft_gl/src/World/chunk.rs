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
        self.Blocks[To1DUsize((0, CHUNK_BOUNDS_Y as usize - 5, 0))] = Block { ID: 6 };
    }

    pub fn GreedyMesh(&mut self, adj: &[Option<*const Chunk>; 4], blockRegistry: &BlockRegistry){
        let dimensions = [CHUNK_BOUNDS_X as usize, CHUNK_BOUNDS_Y as usize, CHUNK_BOUNDS_Z as usize];

        //TODO have this take in a global chunk cord and use it to index into the current chunk or adjacent chunks
        let solid = |x: i32, y: i32, z: i32, d: usize| -> Option<bool> {
            let mut arr = [x, y, z];
            let idx = [(0, 1), (0, 0), (2, 3)]; //adjacent chunk indices
            //let idx = [(0, 1), (0, 0), (0, 1)];
            //println!("{:?} haha {}", arr, d);
            let mut outIdx = -1;
            if arr[d] == -1 {
                arr[d] = dimensions[d] as i32 - 1;
                outIdx = idx[d].0;
            }
            else if arr[d] == dimensions[d] as i32 {
                arr[d] = 0;
                outIdx = idx[d].1;
            }
            //println!("{:?} haha", arr);
            if outIdx == -1_i32 {
                return Some(self.Blocks[To1Di((arr[0], arr[1], arr[2])) as usize] != Block::Air());
            }

            if let Some(dat) = adj[outIdx as usize] {

                return unsafe { Some((*dat).Blocks[To1Di((arr[0], arr[1], arr[2])) as usize] != Block::Air()) };
            }
            None
        };

        let getBlock = |x: &mut [i32; 3], dim: usize| -> Block {
            let temp = x[dim];
            // if temp >= dimensions[dim] as i32 {
            //     println!("THE FUCK??? {:?} dim {} bounds {:?}", x, dim, dimensions);
            // }
            //x[dim] = (x[dim] + 1) / 2;
            x[dim] = (x[dim] - 1).max(0);
            let mut block = self.Blocks[To1Di(( x[0],
                 x[1],
                 x[2])) as usize].clone();
            if block == Block::Air() && x[dim] != dimensions[dim] as i32
            {
                x[dim] += 1;
                block = self.Blocks[To1Di(( x[0],
                    x[1],
                    x[2])) as usize].clone();
                // if block == Block::Air() {
                //     println!("HUH?????? {:?} dim {} and bounds {:?}", x, dim, dimensions);
                // }
            }
            x[dim] = temp;
            block
        };

        //TODO there are 2 faces per dimension. Imagine y, u have top and bottom face. Divide x[d] % 2 to get fID then add d * 2. Do euclid modulo b/c the -1
        let mut addVert = |x: i32, y: i32, z: i32, tID: i32, vID: i32, fID: i32, w: usize, h: usize| {
            let core = ( x  | z << 4 | y << 8 |  (tID as i32) << 16 | (vID as i32) << 24 | (fID as i32) << 26 ) as u32;
            let dims = w | h << 16;

            self.Mesh.push(Vertex { Core: core, Dims: dims as u32} );
        };

        //loop over each axis | x, y, z |
        for dim in 0..3_usize {

            //the two axis we are going to be sweeping over. These create a plane
            //they are guaranteed to \= the current dimensions 'dim'
            let axis1 = (dim + 1) % 3;
            let axis2 = (dim + 2) % 3;

            let mut x: [i32; 3] = [0; 3];
            //a vector in the direction of axis 'dim'
            let mut q: [i32; 3] = [0; 3]; 

            //make a mask over the plane of axis 1 and 2
            let mut mask = vec![false; dimensions[axis1] * dimensions[axis2]];
            q[dim] = 1;

            //create a plane slice of axis 1 and 2 for every value of axis 3, the 'dim' variable
            x[dim] = -1;
            while x[dim] < dimensions[dim] as i32 {

                //make the mask
                let mut n = 0;

                x[axis2] = 0;
                while x[axis2] < dimensions[axis2] as i32 {
                    x[axis1] = 0;
                    while  x[axis1] < dimensions[axis1] as i32 {
                        
                         if x[dim] == -1 && dim == 1 {
                            n += 1;
                            x[axis1] += 1;
                            continue;
                         }
                         
                         if  x[dim] == dimensions[dim] as i32 - 1 && dim == 1 {
                            n += 1;
                            x[axis1] += 1;
                            continue;
                         }
                        
                         //-1 adds an extra iteration
                         //Imagine we had dim = 2 on y axis. Then that's 2 blocks, but 3 faces
                         //Hence the extra iteration

                            //look at curr... (if y < bounds y, don't draw it hence the else {false}. That's the floor)
                         let currB = solid(x[0], x[1], x[2], dim);
                                //and curr + the direction of dim... (allowed to draw the top hence the else {true})
                         let compB =  solid(x[0] + q[0], x[1] + q[1], x[2] + q[2], dim);
                        
                        //True in this case means we draw a face here. This happens if the adjacent blocks solidty are not the same
                        //For example, you wouldnt draw a face between two air or solid blocks. But you would draw one between an air and solid
                        mask[n] = if currB.is_none() || compB.is_none() {false} else {currB != compB};  
                        n += 1;
                        x[axis1] += 1;
                    }

                    x[axis2] += 1;
                }

                x[dim] += 1;

                n = 0;

                //loop over the plane again to construct the mesh
                for j in 0..dimensions[axis2] {
                    let mut i = 0;
                    while i < dimensions[axis1] {

                        //if this point has a solid face
                        if mask[n] {
                            x[axis1] = i as i32;
                            x[axis2] = j as i32;

                            //extend the width of this face as long as there are adjacent faces to the right of axis 1 (as determined by the mask)
                            //If an air block, then the next one will be a solid block
                            let currBlock = getBlock(&mut x, dim);
                            // if currBlock == Block::Air() {
                            //     let mut b = x.clone();
                            //     b[dim] += 1;
                            //     currBlock = getBlock(&mut b, dim);
                            //     if currBlock == Block::Air() {
                            //         println!("WHAT THE FUYCK!!!");
                            //     }
                            // }

                            let mut idx = [x[0], x[1], x[2]];
                            idx[axis1] += 1;

                            let mut w = 1;
                            while i + w < dimensions[axis1] && mask[n + w] && currBlock == getBlock(&mut idx, dim) //TODO don't do &mut just clone it
                            { 
                                w += 1;
                                idx[axis1] += 1;
                            } //TODO only extend with if block is the same
                            

                            //now extend the quad's height along axis2
                            let mut break_ = false;
                            let mut h = 1;
                            while j + h < dimensions[axis2] {

                                //add one to height and reset width
                                idx[axis2] += 1;
                                idx[axis1] = x[axis1];

                                for k in 0..w {
                                    //if there isn't a solid face present, the quad has a hole in it
                                    if ! mask[k + n + h * dimensions[axis1]] || currBlock != getBlock(&mut idx, dim) { //n already includes the offset of j
                                        //perform a double break
                                        break_ = true;
                                        break;
                                    }
                                    idx[axis1] += 1;
                                }

                                if break_ {
                                    break;
                                }

                                h += 1;
                            }

                            //d1 and d2 store the dimensions of each greedily meshed quad
                            let mut d1 = [0; 3];
                            let mut d2 = [0; 3];
                            d1[axis1] = w as i32;
                            d2[axis2] = h as i32;


                            //clear the mask to prevent the creation of duplicate faces
                           
                            //TODO DOES REM EUCLID MODIFY IT????
                            let offsets = [0, 2, 4];
                            let fID = (x[dim] - 1).rem_euclid(2) as i32 + offsets[dim];
                            let mut texID = 7_i32;
                            //TODO textureData should not be an optional. Either its real or the null texture
                            if let Some(TextureData::SixSided(data)) = &blockRegistry.GetAttributesOf(&currBlock).TextureData {
                                texID = data.TextureID as i32 + data.Offsets[fID as usize] as i32;//+ data.Offsets[fID as usize] as i32;
                            } else {
                               // println!("uh oh {} with {:?}", currBlock.ID, x);
                            }

                            
                            let mut offsets = [3, 2, 1, 0];
                            if dim == 0 {
                                let temp = d1;
                                d1 = d2;
                                d2 = temp;
                                let tmp = w;
                                w = h;
                                h = tmp;
                                //offsets = [3, 2, 1, 0];
                                println!("d1 {:?} d2 {:?}", d1, d2);
                            }
                            addVert(x[0], x[1], x[2], texID, offsets[0], fID, w, h);                 // Top-left vertice position
                            addVert(x[0] + d1[0],x[1] + d1[1], x[2] + d1[2], texID, offsets[1], fID, w, h);       // Top right vertice position
                            addVert(x[0] + d2[0],         x[1] + d2[1],         x[2] + d2[2], texID, offsets[2], fID, w, h);        // Bottom left vertice position
                            addVert(x[0] + d1[0] + d2[0], x[1] + d1[1] + d2[1], x[2] + d1[2] + d2[2], texID, offsets[3], fID, w, h);  // Bottom right vertice position
                           // println!("pos || 1 || {} {} {} AND || 2 || {} {} {} with w {} and h {} ", x[0], x[1], x[2], x[0] + d1[0] + d2[0], x[1] + d1[1] + d2[1], x[2] + d1[2] + d2[2], w, h);
                            
                           if dim == 0{
                            let tt = h;
                            h = w;
                            w = tt;
                           }
                        
                            for l in 0..h {
                                for k in 0..w {
                                    mask[n + k + l * dimensions[axis1]] = false;
                                }
                            }
                            i += w;
                            n += w;
            
                        }
                        else {
                            i += 1;
                            n += 1;
                        }
                    }
                }
            }
        }
    }
}
pub fn To3D(idx: u32) -> (u32, u32, u32){
    (idx % CHUNK_BOUNDS_X, idx / (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z), idx % (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Z) / CHUNK_BOUNDS_X)
}

pub fn To1D(cord: (u32, u32, u32)) -> u32{
    cord.0 + CHUNK_BOUNDS_X * (cord.2 + cord.1 * CHUNK_BOUNDS_Z)
}

pub fn To1Di(cord: (i32, i32, i32)) -> i32{
    cord.0 + CHUNK_BOUNDS_X as i32 * (cord.2 + cord.1 * CHUNK_BOUNDS_Z as i32)
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