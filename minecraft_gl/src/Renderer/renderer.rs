use std::collections::HashSet;

use crate::Event::event::Event;
use crate::Scene::camera::Camera;
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource::ResourceManager;
use crate::World::chunk::Chunk;
use super::worldRenderer::WorldRenderer;
use super::spriteRenderer::SpriteRenderer;

pub struct Renderer{
    ResourceManager: ResourceManager,
    WorldRenderer: WorldRenderer,
    SpriteRenderer: SpriteRenderer,
}

impl Renderer{
    pub fn New(blockAtlas: TextureAtlas, itemAtlas: TextureAtlas, device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) -> Self {
        let mut resourceManager = ResourceManager::New();

        let worldRenderer = WorldRenderer::New(&mut resourceManager, blockAtlas, device, queue, config);

        let spriteRenderer = SpriteRenderer::New(&mut resourceManager, itemAtlas, device, queue, config);

        let mut s = Self {
            ResourceManager: resourceManager,
            WorldRenderer: worldRenderer, 
            SpriteRenderer: spriteRenderer,
        };

        s.Init();
        s
    }

    pub fn Init(&mut self){
        //self.WorldRenderer.Init();
       // self.SpriteRenderer.Init();
    }

    pub fn Render(&mut self, chunks: &Vec<Chunk>, renderList: &HashSet<usize>, camera: &Camera, pass: &mut wgpu::RenderPass, queue: &wgpu::Queue){
        self.WorldRenderer.Render(chunks, renderList, camera, pass, queue);
        self.SpriteRenderer.Render(camera, pass);
    }

    pub fn OnEvent(&mut self, _event: &Event){
       // self.SpriteRenderer.OnEvent(event);
    }
}