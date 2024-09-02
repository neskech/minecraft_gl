use super::{
    componentAllocator::{self, ComponentAllocator},
    entity::EntityId,
};
use std::{
    any::{type_name, TypeId},
    borrow::BorrowMut,
    collections::HashMap,
};

pub struct ComponentManager {
    componentAllocators: Vec<ComponentAllocator>,
    typeIdToIndex: HashMap<TypeId, usize>,
}

impl ComponentManager {
    pub fn New() -> ComponentManager {
        ComponentManager {
            componentAllocators: Vec::new(),
            typeIdToIndex: HashMap::new(),
        }
    }

    pub fn GetComponent<T: 'static>(
        &mut self,
        entity: EntityId,
    ) -> Result<&mut T, componentAllocator::Error> {
        let allocator = self.GetAllocator::<T>();
        allocator.GetComponent::<T>(entity)
    }

    pub fn AddComponent<T: 'static>(
        &mut self,
        entity: EntityId,
        component: T,
    ) -> Result<&mut T, componentAllocator::Error> {
        let allocator = self.GetAllocator::<T>();
        allocator.AllocateComponent::<T>(entity, component)?;
        self.GetComponent(entity)
    }

    pub fn HasComponent<T: 'static>(&mut self, entity: EntityId) -> bool {
        let allocator = self.GetAllocator::<T>();
        allocator.HasComponent::<T>(entity)
    }

    pub fn RemoveComponent<T: 'static>(
        &mut self,
        entity: EntityId,
    ) -> Result<(), componentAllocator::Error> {
        let allocator = self.GetAllocator::<T>();
        allocator.FreeComponent::<T>(entity)
    }

    fn GetAllocator<T: 'static>(&mut self) -> &mut ComponentAllocator {
        let typeId = TypeId::of::<T>();
        if !self.typeIdToIndex.contains_key(&typeId) {
            let allocator = ComponentAllocator::New::<T>(type_name::<T>());
            self.componentAllocators.push(allocator);
            self.typeIdToIndex
                .insert(typeId, self.componentAllocators.len() - 1);
        }

        assert!(self.typeIdToIndex.contains_key(&typeId));
        let index = *self.typeIdToIndex.get_mut(&typeId).unwrap();
        &mut self.componentAllocators[index]
    }
}
