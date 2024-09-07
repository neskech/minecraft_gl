use super::{
    componentAllocator::{self, ComponentAllocator},
    entity::EntityId, typeIdMaker::{self, TypeIdMaker},
};
use std::{
    any::{type_name, TypeId},
    borrow::BorrowMut,
    collections::HashMap,
};

pub struct ComponentManager {
    componentAllocators: Vec<ComponentAllocator>,
    componentIdMaker: TypeIdMaker
}

impl ComponentManager {
    pub fn New() -> ComponentManager {
        ComponentManager {
            componentAllocators: Vec::new(),
            componentIdMaker: TypeIdMaker::New()
        }
    }

    pub fn GetComponent<T: 'static>(
        &mut self,
        entityId: EntityId,
    ) -> Result<&mut T, componentAllocator::Error> {
        let allocator = self.GetAllocator::<T>();
        allocator.GetComponent::<T>(entityId)
    }

    pub fn AddComponent<T: 'static>(
        &mut self,
        entityId: EntityId,
        component: T,
    ) -> Result<&mut T, componentAllocator::Error> {
        let allocator = self.GetAllocator::<T>();
        allocator.AllocateComponent::<T>(entityId, component)?;
        self.GetComponent(entityId)
    }

    pub fn HasComponent<T: 'static>(&mut self, entityId: EntityId) -> bool {
        let allocator = self.GetAllocator::<T>();
        allocator.HasComponent::<T>(entityId)
    }

    pub fn RemoveComponent<T: 'static>(
        &mut self,
        entityId: EntityId,
    ) -> Result<(), componentAllocator::Error> {
        let allocator = self.GetAllocator::<T>();
        allocator.FreeComponent::<T>(entityId)
    }

    fn GetAllocator<T: 'static>(&mut self) -> &mut ComponentAllocator {
        if !self.componentIdMaker.HasTypeBeenRegistered::<T>() {
            self.componentIdMaker.RegisterType::<T>();
            let allocator = ComponentAllocator::New::<T>(type_name::<T>());
            self.componentAllocators.push(allocator);
        }

        let index = self.componentIdMaker.GetTypeId::<T>();
        &mut self.componentAllocators[index]
    }
}
