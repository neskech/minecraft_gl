use std::rc::Rc;

use crate::{World::{block::BlockRegistry, item::ItemRegistry}, Event::event::Event};

use super::sceneManager::Scene;
pub struct MainMenu {
    BlockRegistry: Rc<BlockRegistry>,
    ItemRegistry: Rc<ItemRegistry>,
}

impl MainMenu{ 
    pub fn New(blockRegistry: &Rc<BlockRegistry>, itemRegistry: &Rc<ItemRegistry>) -> Self {
        let mut s =Self {
            BlockRegistry: Rc::clone(blockRegistry),
            ItemRegistry: Rc::clone(itemRegistry),
        };

        s.Init();
        s
    }

    fn Init(&mut self) {
        
    }

    pub fn Load(&mut self, savePath: &str) {
        
    }

    pub fn Save(&self) {
        
    }

    pub fn Destroy(&mut self) {
        
    }
}

impl Scene for MainMenu{
    fn Update(&mut self, timeStep: f32) {
        
    }

    fn Render(&mut self, renderer: &mut crate::Renderer::renderer::Renderer, target: &mut glium::Frame) {
        
    }

    fn OnEvent(&mut self, event: &Event) {
        
    }

    fn AsAny(&self) -> &dyn std::any::Any{
        self
    }
    fn AsAnyMut(&mut self) -> &mut dyn std::any::Any{
        self
    }

}