use std::rc::Rc;

use lazy_static::__Deref;

use crate::{Renderer::renderer::Renderer, World::{world, block::BlockRegistry, item::ItemRegistry, crafting::CraftingRegistry}, Event::event::Event};

use super::{worldScene::WorldScene, mainmenu::MainMenu};

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
    BlockRegistry: Rc<BlockRegistry>,
    ItemRegistry: Rc<ItemRegistry>,
    CraftingRegistry: Rc<CraftingRegistry>,
}

impl SceneManager{
    pub fn New(display: &glium::Display) -> Self {

        let mut craftingR = CraftingRegistry::New();

        let itemR = Rc::new(match ItemRegistry::New(&mut craftingR) {
            Ok(val) => val,
            Err(error) => { panic!("Error! Item registry creation failed in scene manager. The error:\n{}", error.to_string()) }
        });

        let blockR = Rc::new(match BlockRegistry::New(&itemR) {
            Ok(val) => val,
            Err(error) => { panic!("Error! Block registry creation failed in scene manager. The error:\n{}", error.to_string()) }
        });

        let mut craftingR = Rc::new(craftingR);

        Self {
            CurrentScene: Box::new(WorldScene::New(&blockR, &itemR, &mut craftingR)),
            CurrentSceneState: SceneState::WorldScene,

            Renderer: Renderer::New(&blockR, &itemR, display),
            BlockRegistry: blockR,
            ItemRegistry: itemR,
            CraftingRegistry: craftingR
        }
    }

    pub fn Update(&mut self, timeStep: f32){
        //self.CurrentScene.Update(timeStep);
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
            MainMenu => {
                let worldScene = self.CurrentScene.AsAnyMut().downcast_mut::<WorldScene>().unwrap();
                worldScene.Save();
                worldScene.Destroy();
                self.CurrentScene = Box::new(MainMenu::New(&self.BlockRegistry, &self.ItemRegistry));
            },
            WorldScene => {
                //TODO add logic for serializtion and deserilaztion
                let mainMenu = self.CurrentScene.AsAnyMut().downcast_mut::<MainMenu>().unwrap();
                mainMenu.Destroy();
                
                let worldScene = WorldScene::New(&self.BlockRegistry, &self.ItemRegistry, &self.CraftingRegistry);
                self.CurrentScene = Box::new(worldScene);
                
            }
        };
    }
}