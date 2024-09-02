use std::{
    collections::{HashSet, VecDeque},
    default,
};
use thiserror::Error;

use super::{
    componentIdMaker::ComponentIdMaker,
    entity::EntityId,
    signature::{Signature, SignatureBuilder},
};

#[derive(Error, Debug)]
#[error("Entity does not exist")]
pub struct EntityDoesNotExist;

#[derive(Debug)]
pub struct EntityData {
    baseEntity: EntityId,
    name: String,
    tag: String,
    signature: Signature,
    children: Vec<EntityId>,
    parent: Option<EntityId>,
}

pub struct EntityManager {
    entityIdQueue: VecDeque<usize>,
    entityData: Vec<EntityData>,
    entityToIndex: Vec<Option<usize>>,
}

impl EntityManager {
    pub fn New() -> EntityManager {
        EntityManager {
            entityIdQueue: VecDeque::new(),
            entityData: Vec::new(),
            entityToIndex: Vec::new(),
        }
    }

    pub fn CreateEntity(&mut self, componentIdMaker: &mut ComponentIdMaker) -> EntityId {
        let numEntities = self.entityData.len();
        let newId = self.entityIdQueue.pop_front().unwrap_or(numEntities);

        let newEntity = EntityId(newId);
        self.entityData.push(EntityData {
            baseEntity: newEntity,
            name: String::new(),
            tag: String::new(),
            signature: SignatureBuilder::New(componentIdMaker).Build(),
            children: Vec::new(),
            parent: None,
        });

        if self.entityToIndex.len() <= newId {
            for i in self.entityToIndex.len()..newId + 1 {
                self.entityToIndex.push(None)
            }
        }
        self.entityToIndex[newId] = Some(self.entityData.len() - 1);

        newEntity
    }

    pub fn DeleteEntity(&mut self, entityId: EntityId) -> Result<(), EntityDoesNotExist> {
        if (!self.IsEntityAlive(entityId)) {
            return Err(EntityDoesNotExist);
        }

        let index = self.entityToIndex[entityId.0].unwrap();
        let last = self.entityData.pop().unwrap();
        let lastEntity = last.baseEntity;
        self.entityData[index] = last;

        assert!(self.IsEntityAlive(lastEntity));
        self.entityToIndex[lastEntity.0] = Some(index);
        self.entityToIndex[entityId.0] = None;
        self.entityIdQueue.push_back(entityId.0);

        Ok(())
    }

    pub fn IsEntityAlive(&self, entityId: EntityId) -> bool {
        let inRange = entityId.0 < self.entityToIndex.len();
        let isSome = self.entityToIndex[entityId.0].is_some();
        inRange && isSome
    }

    pub fn EntityData(&mut self, entityId: EntityId) -> Result<&EntityData, EntityDoesNotExist> {
        if (!self.IsEntityAlive(entityId)) {
            return Err(EntityDoesNotExist);
        }

        let index = self.entityToIndex[entityId.0].unwrap();
        Ok(&self.entityData[index])
    }

    pub fn MutableEntityData(
        &mut self,
        entityId: EntityId,
    ) -> Result<&mut EntityData, EntityDoesNotExist> {
        if (!self.IsEntityAlive(entityId)) {
            return Err(EntityDoesNotExist);
        }

        let index = self.entityToIndex[entityId.0].unwrap();
        Ok(&mut self.entityData[index])
    }
}
