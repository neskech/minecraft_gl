use crate::{World::item::ItemAttribute, Event::event::Event};


pub struct ItemBehavior{
    //TODO add an OnCreate() function that handles initialization of some of the item fields in its attribute map
    pub OnLeftClick: fn(attributes: &ItemAttribute),
    pub OnRightClick: fn(attributes: &ItemAttribute) -> Option<fn(attributes: &ItemAttribute, Event)>,
    pub CustomBehavor: Option<fn(attributes: &ItemAttribute, Event)>, //returned in onRightClick
}

impl Default for ItemBehavior{
    fn default() -> Self {
        #[allow(unused)]
        fn onLeft(attributes: &ItemAttribute){

        }
        #[allow(unused)]
        fn onRight(attributes: &ItemAttribute) -> Option<fn(attributes: &ItemAttribute, Event)>{
            None
        }

        Self { OnLeftClick: onLeft, OnRightClick: onRight, CustomBehavor: None }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
// ///////////////////////////////////////////////////////////////////////////////////////////////./

// crate::CreateBinding!(ItemBindingFunction,


// )