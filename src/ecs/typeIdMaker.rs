use std::{any::TypeId, collections::HashMap};

pub struct TypeIdMaker {
    typeIdToIndex: HashMap<TypeId, usize>,
}

impl TypeIdMaker {
    pub fn New() -> TypeIdMaker {
        TypeIdMaker {
            typeIdToIndex: HashMap::new(),
        }
    }

    pub fn GetTypeId<T: 'static>(&mut self) -> usize {
        let typeId = TypeId::of::<T>();
        if !self.typeIdToIndex.contains_key(&typeId) {
            self.typeIdToIndex.insert(typeId, self.typeIdToIndex.len());
        }

        *self.typeIdToIndex.get(&typeId).unwrap()
    }

    pub fn RegisterType<T: 'static>(&mut self) {
        let typeId = TypeId::of::<T>();
        if !self.typeIdToIndex.contains_key(&typeId) {
            self.typeIdToIndex.insert(typeId, self.typeIdToIndex.len());
        }
    }

    pub fn HasTypeBeenRegistered<T: 'static>(&self) -> bool {
        let typeId = TypeId::of::<T>();
        self.typeIdToIndex.contains_key(&typeId)
    }
}
