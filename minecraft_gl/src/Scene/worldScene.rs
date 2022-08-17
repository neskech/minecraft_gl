
use crate::{World::{block::BlockRegistry, world::World, item::ItemRegistry, crafting::CraftingRegistry}, Event::event::Event};
use super::{sceneManager::Scene, camera::Camera};

pub struct WorldScene{
    World: World,
    Camera: Camera
}

impl<'a> WorldScene{
    pub fn New(blockRegistry: BlockRegistry, itemRegistry: ItemRegistry, craftingRegistry: CraftingRegistry) -> WorldScene {
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

    pub fn Load(&mut self, _savePath: &str) {

    }

    pub fn Destroy(&mut self) {
        
    }

}
impl Scene for WorldScene{
    fn Update(&mut self, _timeStep: f32) {
        self.World.Update((self.Camera.Position.x, self.Camera.Position.z), &self.Camera)
    }

    fn Render(&mut self, renderer: &mut crate::Renderer::renderer::Renderer, target: &mut glium::Frame) {
        renderer.Render(&self.World.RenderList, &self.Camera, target);
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