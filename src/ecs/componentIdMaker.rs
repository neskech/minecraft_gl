use std::{any::TypeId, collections::HashMap};

pub struct ComponentIdMaker {
    typeIdToIndex: HashMap<TypeId, usize>,
}

impl ComponentIdMaker {
    pub fn New() -> ComponentIdMaker {
        ComponentIdMaker {
            typeIdToIndex: HashMap::new()
        }
    }

    pub fn GetComponentId<T: 'static>(&mut self) -> usize {
        let typeId = TypeId::of::<T>();
        if !self.typeIdToIndex.contains_key(&typeId) {
            self.typeIdToIndex.insert(typeId, self.typeIdToIndex.len());
        }

        *self.typeIdToIndex.get(&typeId).unwrap()
    }
}
