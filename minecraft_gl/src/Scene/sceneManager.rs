use crate::{Renderer::renderer::Renderer, World::{block::BlockRegistry, item::ItemRegistry, crafting::CraftingRegistry, ReadAttributes}, Event::event::Event};
use super::{worldScene::WorldScene, mainmenu::MainMenu};
use crate::Renderer::worldRenderer::BLOCK_TEXTURE_RESOLUTION;

#[derive(PartialEq, Eq, Debug)]
pub enum SceneState{
    MainMenu,
    WorldScene,
}
pub trait Scene{
    fn Update(&mut self, timeStep: f32);
    fn Render(&mut self, renderer: &mut Renderer, target: &mut glium::Frame);
    fn OnEvent(&mut self, event: &Event);

    fn AsAny(&self) -> &dyn std::any::Any;
    fn AsAnyMut(&mut self) -> &mut dyn std::any::Any;
}

pub struct SceneManager{
    CurrentScene: Box<dyn Scene>,
    CurrentSceneState: SceneState,

    Renderer: Renderer,
}

impl SceneManager{
    pub fn New(display: &glium::Display) -> Self {

        let mut craftingR = CraftingRegistry::New();
        let mut itemR = ItemRegistry::New(); 
        let mut blockR = BlockRegistry::New();

        match ReadAttributes(&mut blockR, &mut itemR, &mut craftingR) {
            Err(msg) => {
                panic!("Error! Attribute reading failed for registries. The error:\n{}", msg.to_string());
            },
            _ => {}
        };

        let blockAtlas = match blockR.GenerateAtlas(BLOCK_TEXTURE_RESOLUTION, display) {
            Ok(val) => val,
            Err(msg) => {
                  panic!("Error! World renderer creation failed due to block atlas creation. The error:\n{}.", msg);
            }
        };

        let itemAtlas = match itemR.GenerateAtlas(BLOCK_TEXTURE_RESOLUTION, display) {
            Ok(val) => val,
            Err(msg) => {
                panic!("Error! Sprite renderer creation failed due to item atlas creation. The error:\n{}.", msg);
            }
        };

        Self {
            CurrentScene: Box::new(WorldScene::New(blockR, itemR,  craftingR)),
            CurrentSceneState: SceneState::WorldScene,
            //TODO create the atlases here and dont worry about passing the registrys down to the renderer
            //TODO also prevent the mainMenu from having the registries, I dont care
            Renderer: Renderer::New(blockAtlas, itemAtlas, display),
        }
    }

    pub fn Update(&mut self, timeStep: f32){
        self.CurrentScene.Update(timeStep);
    }

    pub fn Render(&mut self, target: &mut glium::Frame){
        self.CurrentScene.Render(&mut self.Renderer, target);
    }

    pub fn OnEvent(&mut self, event: &Event){
        //propogate down application events to the current scene
        //deal with scene manager events like a transition event from the SceneManagerBus

        self.CurrentScene.OnEvent(event);
        self.Renderer.OnEvent(event);
    }

    pub fn OnTransition(&mut self, state: SceneState){
        if self.CurrentSceneState == state {
            //TODO have some way of printing the scene state
            panic!("Error! Cannot transition to the same scene state type. Consider changing your logic to prevent this from happening!");
        }

        match state {
            SceneState::MainMenu => {
                let worldScene = self.CurrentScene.AsAnyMut().downcast_mut::<WorldScene>().unwrap();
                worldScene.Save();
                worldScene.Destroy();
                self.CurrentScene = Box::new(MainMenu::New());
            },
            SceneState::WorldScene => {
                //TODO add logic for serializtion and deserilaztion
                let mainMenu = self.CurrentScene.AsAnyMut().downcast_mut::<MainMenu>().unwrap();
                mainMenu.Destroy();

                let mut craftingR = CraftingRegistry::New();
                let mut itemR = ItemRegistry::New(); 
                let mut blockR = BlockRegistry::New();
        
                match ReadAttributes(&mut blockR, &mut itemR, &mut craftingR) {
                    Err(msg) => {
                        panic!("Error! Attribute reading failed for registries. The error:\n{}", msg.to_string());
                    },
                    _ => {}
                };
                
                let worldScene = WorldScene::New(blockR, itemR,  craftingR);
                self.CurrentScene = Box::new(worldScene);
                
            }
        };
    }
}