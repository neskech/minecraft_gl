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

pub trait IComponentAllocator {}

#[derive(Debug)]
pub struct ComponentAllocator<T: Copy> {
    components: Vec<T>,
    entityToIndex: HashMap<Entity, usize>,
    indexToEntity: HashMap<usize, Entity>,
    componentTypeName: String,
}

impl<T: Copy> IComponentAllocator for ComponentAllocator<T> {}

impl<T: Copy> ComponentAllocator<T> {
    pub fn New(componentTypeName: impl Into<String>) -> ComponentAllocator<T> {
        ComponentAllocator {
            components: Vec::new(),
            entityToIndex: HashMap::new(),
            indexToEntity: HashMap::new(),
            componentTypeName: componentTypeName.into(),
        }
    }

    pub fn AllocateComponent(&mut self, entity: Entity, component: T) -> Result<(), Error> {
        if self.components.len() >= MAX_COMPONENTS {
            return Err(Error::MaximumComponentsExceeded);
        }

        if self.entityToIndex.contains_key(&entity) {
            return Err(Error::ComponentAlreadyAllocated);
        }

        let newIndex = self.components.len();
        self.entityToIndex.insert(entity, newIndex);
        self.indexToEntity.insert(newIndex, entity);
        self.components.push(component);

        Ok(())
    }

    pub fn FreeComponent(&mut self, entity: Entity) -> Result<(), Error> {
        let currentIndex = *self
            .entityToIndex
            .get(&entity)
            .ok_or(Error::EntityNotFound)?;

        debug_assert!(currentIndex <= self.components.len() - 1);
        self.components[currentIndex] = self.components[self.components.len() - 1];

        let lastEntityId = *self
            .indexToEntity
            .get(&(self.components.len() - 1))
            .ok_or(Error::EntityNotFound)?;
        self.entityToIndex.insert(lastEntityId, currentIndex);
        self.indexToEntity.insert(currentIndex, lastEntityId);

        self.indexToEntity.remove(&(self.components.len() - 1));
        self.entityToIndex.remove(&entity);

        self.components.pop();

        Ok(())
    }

    pub fn GetComponent(&mut self, entity: Entity) -> Result<&mut T, Error> {
        let index = *self
            .entityToIndex
            .get(&entity)
            .ok_or(Error::EntityNotFound)?;
        Ok(&mut self.components[index])
    }

    pub fn GetComponentTypeName(&self) -> &str {
        &self.componentTypeName
    }
}
