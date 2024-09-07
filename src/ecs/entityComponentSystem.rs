use thiserror::Error;

use super::{
    componentAllocator,
    componentManager::ComponentManager,
    entity::EntityId,
    entityManager::{self, EntityManager},
    layerManager::LayerManager,
    systemManager::SystemManager,
    typeIdMaker::TypeIdMaker,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Component allocator encountered an error")]
    ComponentAllocationError(#[from] componentAllocator::Error),
    #[error("Component allocator encountered an error")]
    EntityError(#[from] entityManager::EntityDoesNotExist),
}

pub struct EntityComponentSystem {
    componentManager: ComponentManager,
    entityManager: EntityManager,
    systemManager: SystemManager,
    layerManager: LayerManager,
    componentIdMaker: TypeIdMaker,
}

impl EntityComponentSystem {
    pub fn New() -> EntityComponentSystem {
        EntityComponentSystem {
            componentManager: ComponentManager::New(),
            entityManager: EntityManager::New(),
            systemManager: SystemManager::New(),
            layerManager: LayerManager::New(),
            componentIdMaker: TypeIdMaker::New(),
        }
    }

    pub fn CreateEntity(&mut self) -> Result<EntityId, Error> {
        let entityId = self.entityManager.CreateEntity(&mut self.componentIdMaker);
        let entityData = self.entityManager.MutableEntityData(entityId)?;
        self.systemManager
            .OnEntityCreated(entityId, entityData.signature.clone());
        Ok(entityId)
    }

    pub fn DeleteEntity(&mut self, entityId: EntityId) -> Result<(), Error> {
        let entityData = self.entityManager.MutableEntityData(entityId)?;
        self.systemManager
            .OnEntityDestroyed(entityId, entityData.signature.clone());
        self.entityManager.DeleteEntity(entityId)?;
        Ok(())
    }

    pub fn AddComponent<T: 'static>(
        &mut self,
        entityId: EntityId,
        component: T,
    ) -> Result<&mut T, Error> {
        let reference = self
            .componentManager
            .AddComponent::<T>(entityId, component)?;

        let entityData = self.entityManager.MutableEntityData(entityId)?;
        entityData
            .signature
            .AddComponent::<T>(&mut self.componentIdMaker);

        self.systemManager
            .OnEntitySignatureChanged(entityId, entityData.signature.clone());

        Ok(reference)
    }

    pub fn RemoveComponent<T: 'static>(&mut self, entityId: EntityId) -> Result<(), Error> {
        let reference = self.componentManager.RemoveComponent::<T>(entityId)?;

        let entityData = self.entityManager.MutableEntityData(entityId)?;
        entityData
            .signature
            .RemoveComponent::<T>(&mut self.componentIdMaker);

        self.systemManager
            .OnEntitySignatureChanged(entityId, entityData.signature.clone());

        Ok(reference)
    }

    pub fn SetName(&mut self, entityId: EntityId, name: impl Into<String>) -> Result<(), Error> {
        let entityData = self.entityManager.MutableEntityData(entityId)?;
        entityData.name = name.into();
        Ok(())
    }

    pub fn SetTag(&mut self, entityId: EntityId, tag: impl Into<String>) -> Result<(), Error> {
        let entityData = self.entityManager.MutableEntityData(entityId)?;
        entityData.tag = tag.into();
        Ok(())
    }

    pub fn GetName(&self, entityId: EntityId) -> Result<&str, Error> {
        let entityData = self.entityManager.EntityData(entityId)?;
        Ok(&entityData.name)
    }

    pub fn GetTag(&mut self, entityId: EntityId, tag: impl Into<String>) -> Result<&str, Error> {
        let entityData = self.entityManager.EntityData(entityId)?;
        Ok(&entityData.tag)
    }

    pub fn AddChildToEntity(&mut self, parentId: EntityId, childId: EntityId) -> Result<(), Error> {
        if !self.entityManager.IsEntityAlive(childId) {
            return Err(Error::EntityError(entityManager::EntityDoesNotExist));
        }

        let entityData = self.entityManager.MutableEntityData(parentId)?;
        entityData.children.push(childId);
        Ok(())
    }

    pub fn RemoveChildFromEntity(
        &mut self,
        parentId: EntityId,
        childId: EntityId,
    ) -> Result<(), Error> {
        if !self.entityManager.IsEntityAlive(childId) {
            return Err(Error::EntityError(entityManager::EntityDoesNotExist));
        }

        let entityData = self.entityManager.MutableEntityData(parentId)?;
        entityData.children.retain(|id| *id != childId);
        Ok(())
    }

    pub fn GetChildren(&self, entityId: EntityId) -> Result<&Vec<EntityId>, Error> {
        let entityData = self.entityManager.EntityData(entityId)?;
        Ok(&entityData.children)
    }

    pub fn GetParent(&self, entityId: EntityId) -> Result<Option<EntityId>, Error> {
        let entityData = self.entityManager.EntityData(entityId)?;
        Ok(entityData.parent)
    }

    pub fn AddLayer(&mut self, layerName: &str) {
        self.layerManager.AddLayer(layerName);
    }

    pub fn AddEntityToLayer(&mut self, entityId: EntityId, layerName: &str) -> Result<(), Error> {
        assert!(self.layerManager.HasLayer(layerName));
        let entityData = self.entityManager.MutableEntityData(entityId)?;
        self.layerManager
            .AddLayerToMask(layerName, &mut entityData.layerMask);
        Ok(())
    }

    pub fn AddEntityToLayers(
        &mut self,
        entityId: EntityId,
        layerNames: Vec<&str>,
    ) -> Result<(), Error> {
        layerNames
            .iter()
            .for_each(|name| assert!(self.layerManager.HasLayer(name)));
        let entityData = self.entityManager.MutableEntityData(entityId)?;
        self.layerManager
            .AddLayersToMask(layerNames, &mut entityData.layerMask);
        Ok(())
    }
}
