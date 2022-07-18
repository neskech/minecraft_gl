
use crate::Event::event::Event;
use super::sceneManager::Scene;
pub struct MainMenu {
}

impl MainMenu{ 
    pub fn New() -> Self {
        let mut s =Self {

        };

        s.Init();
        s
    }

    fn Init(&mut self) {
        
    }

    pub fn Load(&mut self, _savePath: &str) {
        
    }

    pub fn Save(&self) {
        
    }

    pub fn Destroy(&mut self) {
        
    }
}

impl Scene for MainMenu{
    fn Update(&mut self, _timeStep: f32) {
        
    }

    fn Render(&mut self, _renderer: &mut crate::Renderer::renderer::Renderer, _target: &mut glium::Frame) {
        
    }

    fn OnEvent(&mut self, _event: &Event) {
        
    }

    fn AsAny(&self) -> &dyn std::any::Any{
        self
    }
    fn AsAnyMut(&mut self) -> &mut dyn std::any::Any{
        self
    }

}