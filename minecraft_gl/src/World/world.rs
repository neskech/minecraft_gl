use super::{block::BlockRegistry, chunk::Chunk, item::ItemRegistry, crafting::CraftingRegistry};


const DEFAULT_RENDER_DISTANCE: usize = 1;
const MAX_CHUNK_GENERATION_PER_FRAME: usize = 3;
const MAX_CHUNK_REMESH_PER_FRAME: usize = 3;

pub struct World{
    pub Chunks: Vec<Chunk>,
    RenderDistance: usize,
    BlockRegistry: BlockRegistry,
    ItemRegistry: ItemRegistry,
    CraftingRegistry: CraftingRegistry,

    RegenerationList: Vec<usize>,
    RemeshList: Vec<usize>,
    pub RenderList: Vec<usize>,
}

impl World{
    pub fn New(craftingRegistry: CraftingRegistry, blockRegistry: BlockRegistry, itemRegistry: ItemRegistry) -> Self{
        let mut chunks: Vec<Chunk> = Vec::new();
        chunks.reserve(DEFAULT_RENDER_DISTANCE * DEFAULT_RENDER_DISTANCE);
        chunks.push(Chunk::OfHeight(5, (0, 0)));
        chunks[0].GenerateMesh(&blockRegistry);
        
        let mut renderList: Vec<usize> = Vec::new();
        renderList.push(0);

        Self{
            Chunks: chunks,
            RenderDistance: DEFAULT_RENDER_DISTANCE,
            BlockRegistry: blockRegistry,
            ItemRegistry: itemRegistry,
            CraftingRegistry: craftingRegistry,

            RegenerationList: Vec::new(),
            RemeshList: Vec::new(),
            RenderList: renderList,
        }
    }
}