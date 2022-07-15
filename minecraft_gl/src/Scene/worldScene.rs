use std::rc::Rc;

use crate::{World::{block::BlockRegistry, world::World, item::ItemRegistry, crafting::CraftingRegistry}, Event::event::Event};

use super::{sceneManager::Scene, camera::Camera};

pub struct WorldScene{
    World: World,
    Camera: Camera
}

impl<'a> WorldScene{
    pub fn New(blockRegistry: &Rc<BlockRegistry>, itemRegistry: &Rc<ItemRegistry>, craftingRegistry: &Rc<CraftingRegistry>) -> WorldScene {
        let mut s = Self {  
            World: World::New(craftingRegistry, blockRegistry, itemRegistry),
            Camera: Camera::New(),
        };

        s.Init();
        s
    }

    fn Init(&mut self) {
        
    }

    pub fn Save(&self) {

    }

    pub fn Load(&mut self, savePath: &str) {

    }

    pub fn Destroy(&mut self) {
        
    }

}
impl Scene for WorldScene{
    fn Update(&mut self, timeStep: f32) {
        
    }

    fn Render(&self, renderer: &crate::Renderer::renderer::Renderer) {
        renderer.Render(&self.World.Chunks, &self.World.RenderList, &self.Camera);
    }

    fn OnEvent(&mut self, event: &Event) {
        self.Camera.OnEvent(event);
    }

    fn AsAny(& self) -> & dyn std::any::Any{
        self
    }
    fn AsAnyMut(& mut self) -> & mut dyn std::any::Any{
        self
    }
}