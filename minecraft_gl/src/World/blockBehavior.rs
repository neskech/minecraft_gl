

use std::collections::HashMap;
use glfw::WindowEvent;
use crate::World::block::BlockAttribute;

use super::item::Item;
use super::block::{BlockRegistry};


pub struct BlockBehavior{
    //TODO add an OnPlace() function which handles if the block should be placed in a chunk's State
    //TODO and also the initialization of the attribute hashmap's fields
    //TODO perhaps make it return an Option<> of its hashmap. If some, add to chunk State
    pub OnLeftClick: fn(attributes: &BlockAttribute, hit: Item),
    pub OnRightClick: fn(attributes: &BlockAttribute) -> Option<fn(attributes: &BlockAttribute, WindowEvent) -> bool>,
    //the bool is for if the window / behavior should close / stop
    CustomBehavor: Option<fn(attributes: &BlockAttribute, WindowEvent) -> bool>, //returned in onRightClick
}

impl Default for BlockBehavior{
    fn default() -> Self {

        #[allow(unused)]
        fn onLeft(attributes: &BlockAttribute, hit: Item){

        }

        #[allow(unused)]
        fn onRight(attributes: &BlockAttribute) -> Option<fn(attributes: &BlockAttribute, WindowEvent) -> bool>{
            None
        }

        Self { OnLeftClick: onLeft, OnRightClick: onRight, CustomBehavor: None }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// ///////////////////////////////////////////////////////////////////////////////////////////////./

pub fn BindCraftingTable(nameToID: HashMap<&str, u8>, registry: &mut BlockRegistry) {
    let craftingTableID = nameToID.get("craftingTable").unwrap();
    #[allow(unused)]
    fn onLeft(attributes: &BlockAttribute, hit: Item){

    }
    
    #[allow(unused)]
    fn onRight(attributes: &BlockAttribute) -> Option<fn(attributes: &BlockAttribute, WindowEvent) -> bool>{
        None
    }

    #[allow(unused)]
    fn CustomBehavior(attributes: &BlockAttribute, inputEvent: WindowEvent) -> bool{
        false
    }

    let behavior = BlockBehavior { OnLeftClick: onLeft, OnRightClick: onRight, CustomBehavor: Some(CustomBehavior) };
    registry.BlockBehaviors[*craftingTableID as usize] = Some(behavior); //move into the vector
}