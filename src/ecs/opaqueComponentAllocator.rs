use super::util::opaqueVector::OpaqueVector;
use thiserror::Error;

use super::entity::Entity;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Component has already been allocated for this entity")]
    ComponentAlreadyAllocated,
    #[error("Entity not found in component allocator")]
    EntityNotFound,
    #[error("Maximum number of components have been exceeded")]
    MaximumComponentsExceeded,
}

const MAX_COMPONENTS: usize = 5128;

#[derive(Debug)]
pub struct OpaqueComponentAllocator {
    components: OpaqueVector,
    entityToIndex: HashMap<Entity, usize>,
    indexToEntity: HashMap<usize, Entity>,
    componentTypeName: String,
}

impl OpaqueComponentAllocator {
    pub fn New<T: 'static>(componentTypeName: impl Into<String>) -> OpaqueComponentAllocator {
        OpaqueComponentAllocator {
            components: OpaqueVector::New::<T>(10),
            entityToIndex: HashMap::new(),
            indexToEntity: HashMap::new(),
            componentTypeName: componentTypeName.into(),
        }
    }

    pub fn AllocateComponent<T: 'static>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<(), Error> {
        if self.components.GetLength() >= MAX_COMPONENTS {
            return Err(Error::MaximumComponentsExceeded);
        }

        if self.entityToIndex.contains_key(&entity) {
            return Err(Error::ComponentAlreadyAllocated);
        }

        let newIndex = self.components.GetLength();
        self.entityToIndex.insert(entity, newIndex);
        self.indexToEntity.insert(newIndex, entity);
        self.components.Push(component);

        Ok(())
    }

    pub fn FreeComponent<T: 'static>(&mut self, entity: Entity) -> Result<(), Error> {
        let currentIndex = *self
            .entityToIndex
            .get(&entity)
            .ok_or(Error::EntityNotFound)?;

        let lastIndex = self.components.GetLength() - 1;
        debug_assert!(currentIndex <= lastIndex);
        *self.components.Get(currentIndex) = self.components.Pop::<T>();

        let lastEntityId = *self
            .indexToEntity
            .get(&lastIndex)
            .ok_or(Error::EntityNotFound)?;
        self.entityToIndex.insert(lastEntityId, currentIndex);
        self.indexToEntity.insert(currentIndex, lastEntityId);

        self.indexToEntity
            .remove(&lastIndex);
        self.entityToIndex.remove(&entity);

        Ok(())
    }

    pub fn HasComponent<T: 'static>(&self, entity: Entity) -> bool {
        self.entityToIndex.contains_key(&entity)
    }

    pub fn GetComponent<T: 'static>(&mut self, entity: Entity) -> Result<&mut T, Error> {
        let index = *self
            .entityToIndex
            .get(&entity)
            .ok_or(Error::EntityNotFound)?;
        Ok(self.components.Get(index))
    }

    pub fn GetComponentTypeName(&self) -> &str {
        &self.componentTypeName
    }
}
