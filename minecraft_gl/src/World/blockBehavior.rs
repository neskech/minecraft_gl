use crate::Event::event::Event;
use crate::World::block::BlockAttribute;
use super::item::Item;
use super::block::BlockRegistry;


pub struct BlockBehavior{
    //TODO add an OnPlace() function which handles if the block should be placed in a chunk's State
    //TODO and also the initialization of the attribute hashmap's fields
    //TODO perhaps make it return an Option<> of its hashmap. If some, add to chunk State
    pub OnLeftClick: fn(attributes: &BlockAttribute, hit: Item),
    pub OnRightClick: fn(attributes: &BlockAttribute) -> Option<fn(attributes: &BlockAttribute, Event) -> bool>,
    //the bool is for if the window / behavior should close / stop
    CustomBehavor: Option<fn(attributes: &BlockAttribute, Event) -> bool>, //returned in onRightClick
}

impl Default for BlockBehavior{
    fn default() -> Self {

        #[allow(unused)]
        fn onLeft(attributes: &BlockAttribute, hit: Item){

        }

        #[allow(unused)]
        fn onRight(attributes: &BlockAttribute) -> Option<fn(attributes: &BlockAttribute, Event) -> bool>{
            None
        }

        Self { OnLeftClick: onLeft, OnRightClick: onRight, CustomBehavor: None }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// ///////////////////////////////////////////////////////////////////////////////////////////////./

crate::CreateBinding!(BlockBindingFunction,

    pub fn BindCraftingTable(registry: &mut BlockRegistry) {
        let craftingTableID = registry.NameToID("craftingTable").expect("could not find crafting table");
        #[allow(unused)]
        fn onLeft(attributes: &BlockAttribute, hit: Item){

        }
        
        #[allow(unused)]
        fn onRight(attributes: &BlockAttribute) -> Option<fn(attributes: &BlockAttribute, Event) -> bool>{
            None
        }

        #[allow(unused)]
        fn CustomBehavior(attributes: &BlockAttribute, inputEvent: Event) -> bool{
            false
        }

        let behavior = BlockBehavior { OnLeftClick: onLeft, OnRightClick: onRight, CustomBehavor: Some(CustomBehavior) };
        registry.BlockBehaviors.insert(craftingTableID, behavior); //move into the vector
    }

);

