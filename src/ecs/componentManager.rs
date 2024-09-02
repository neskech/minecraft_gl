use std::{
    any::{type_name, TypeId},
    borrow::BorrowMut,
    collections::HashMap,
};

use super::{
    componentAllocator::{ComponentAllocator, IComponentAllocator},
    entity::Entity,
    opaqueComponentAllocator::OpaqueComponentAllocator,
};

pub struct ComponentManager {
    componentAllocators: Vec<OpaqueComponentAllocator>,
    typeIdToIndex: HashMap<TypeId, usize>,
}

impl ComponentManager {
    pub fn New() -> ComponentManager {
        ComponentManager {
            componentAllocators: Vec::new(),
            typeIdToIndex: HashMap::new(),
        }
    }

    pub fn GetComponent<T: 'static>(&mut self, entity: Entity) -> &mut T {
        let allocator = self.GetAllocator::<T>();
        allocator.GetComponent::<T>(entity).unwrap()
    }

    pub fn AddComponent<T: 'static>(&mut self, entity: Entity, component: T) -> &mut T {
        let allocator = self.GetAllocator::<T>();
        allocator.AllocateComponent::<T>(entity, component).unwrap();
        self.GetComponent(entity)
    }

    pub fn HasComponent<T: 'static>(&mut self, entity: Entity) -> bool {
        let allocator = self.GetAllocator::<T>();
        allocator.HasComponent::<T>(entity)
    }

    pub fn RemoveComponent<T: 'static>(&mut self, entity: Entity) {
        let allocator = self.GetAllocator::<T>();
        allocator.FreeComponent::<T>(entity).unwrap();
    }

    fn GetAllocator<T: 'static>(&mut self) -> &mut OpaqueComponentAllocator {
        let typeId = TypeId::of::<T>();
        if !self.typeIdToIndex.contains_key(&typeId) {
            let allocator = OpaqueComponentAllocator::New::<T>(type_name::<T>());
            self.componentAllocators.push(allocator);
            self.typeIdToIndex
                .insert(typeId, self.componentAllocators.len() - 1);
        }

        assert!(self.typeIdToIndex.contains_key(&typeId));
        let index = *self.typeIdToIndex.get_mut(&typeId).unwrap();
        &mut self.componentAllocators[index]
    }
}
