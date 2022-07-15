use glfw::WindowEvent;

use crate::World::item::ItemAttribute;


pub struct ItemBehavior{
    //TODO add an OnCreate() function that handles initialization of some of the item fields in its attribute map
    pub OnLeftClick: fn(attributes: &ItemAttribute),
    pub OnRightClick: fn(attributes: &ItemAttribute) -> Option<fn(attributes: &ItemAttribute, WindowEvent)>,
    pub CustomBehavor: Option<fn(attributes: &ItemAttribute, WindowEvent)>, //returned in onRightClick
}

impl Default for ItemBehavior{
    fn default() -> Self {
        #[allow(unused)]
        fn onLeft(attributes: &ItemAttribute){

        }
        #[allow(unused)]
        fn onRight(attributes: &ItemAttribute) -> Option<fn(attributes: &ItemAttribute, WindowEvent)>{
            None
        }

        Self { OnLeftClick: onLeft, OnRightClick: onRight, CustomBehavor: None }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// ///////////////////////////////////////////////////////////////////////////////////////////////./

