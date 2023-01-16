use std::{collections::HashMap, sync::Arc};
use crate::{Renderer::worldRenderer::Vertex, Util::greedyMeshHelper};
use super::{block::{Block, BlockRegistry, TextureData}, State, 
            biomeGenerator::{Biome, BiomeGenerator}
           };

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
        //TODO make an onDestroy() function in blockBehaviors and call it here. 
        //TODO It should take in the block state as a parameter before you delete it
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

        //get rid of straggler stone block
        //TODO figure out why the straggler is being generated
        self.Blocks[To1Di((0, 55,0)) as usize] = Block::Air();
    }

    pub fn GreedyMesh(&mut self, adj: &[Option<Arc<Chunk>>; 4], blockRegistry: &BlockRegistry){
        self.ClearMesh();
        let dimensions = [CHUNK_BOUNDS_X as usize, CHUNK_BOUNDS_Y as usize, CHUNK_BOUNDS_Z as usize];

        for dim in 0..3 {
            greedyMeshHelper::SweepVolume(self, &dimensions, 
                            dim, adj, blockRegistry);
        }
        self.MeshFlora(blockRegistry);
    }

    pub fn MeshFlora(&mut self, blockRegistry: &BlockRegistry) {
        //TODO for this to not have those wierd white squares show up we must have proper draw order
        //TODO drawing the flora LAST ensures proper draw in order within the chunk but not 
        //TODO in regards to other chunks. Draw the other chunks in order of distance from camera
        for x in 0..CHUNK_BOUNDS_X {
            for y in 0..CHUNK_BOUNDS_Y {
                for z in 0..CHUNK_BOUNDS_Z {

                    let p = [x as i32, y as i32, z as i32];
                    let block = self.Blocks[greedyMeshHelper::To1D(&p) as usize];
                    
                    //TODO fix flower rendering
                    let mut texID = -1;
                    if let TextureData::Single(data) = &blockRegistry.GetAttributesOf(&block).TextureData {
                        texID = data.TextureID as i32;
                    } 

                    if greedyMeshHelper::IsFlora(block, blockRegistry) {
                        let pp = greedyMeshHelper::AddArrayVector(&p, &[0, 1, 0]);
                        greedyMeshHelper::ConstructFloraFaces(self, &pp, texID);
                    }
                }
            }
        }
    }
}

pub fn To1D(cord: (u32, u32, u32)) -> u32 {
    cord.0 + CHUNK_BOUNDS_X * (cord.2 + cord.1 * CHUNK_BOUNDS_Z)
}

pub fn To1Di(cord: (i32, i32, i32)) -> i32 {
    cord.0 + CHUNK_BOUNDS_X as i32 * (cord.2 + cord.1 * CHUNK_BOUNDS_Z as i32)
}

fn To1DUsize(cord: (usize, usize, usize)) -> usize {
    cord.0 + CHUNK_BOUNDS_X as usize * (cord.2 + cord.1 * CHUNK_BOUNDS_Z as usize)
}
