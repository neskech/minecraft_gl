use core::panic;
use std::rc::Rc;

use crate::Event::event::Event;
use crate::Scene::camera::Camera;
use crate::Util::resource::ResourceManager;
use crate::World::block::BlockRegistry;
use crate::World::chunk::Chunk;
use crate::World::item::ItemRegistry;
use super::worldRenderer::WorldRenderer;
use super::spriteRenderer::SpriteRenderer;


pub struct Renderer{
    ResourceManager: ResourceManager,
    WorldRenderer: WorldRenderer,
    SpriteRenderer: SpriteRenderer,
}

impl Renderer{
    pub fn New(blockRegistry: &BlockRegistry, itemRegistry: &ItemRegistry) -> Self {
        let mut resourceManager = ResourceManager::New();

        let worldRenderer = match WorldRenderer::New(&mut resourceManager, blockRegistry) {
            Ok(val) => val,
            Err(msg) => panic!("Error! World renderer construction failed! The error:\n{}", msg) //TODO be more specific. What failed?
        };

        let spriteRenderer = match SpriteRenderer::New(&mut resourceManager, itemRegistry) {
            Ok(val) => val,
            Err(msg) => panic!("Error! Sprite renderer construction failed! The error:\n{}", msg)
        };

        let mut s = Self {
            ResourceManager: resourceManager,
            WorldRenderer: worldRenderer, 
            SpriteRenderer: spriteRenderer,
        };

        s.Init();
        s
    }

    pub fn Init(&mut self){
        self.WorldRenderer.Init();
       // self.SpriteRenderer.Init();
    }

    pub fn Render(&self, chunks: &Vec<Chunk>, renderList: &Vec<usize>, camera: &Camera){
        self.WorldRenderer.Render(chunks, renderList, camera);
       // self.SpriteRenderer.Render();
    }

    pub fn OnEvent(&mut self, event: &Event){
       // self.SpriteRenderer.OnEvent(event);
    }
}