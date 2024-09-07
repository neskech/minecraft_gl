use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use thiserror::Error;

use super::{entity::EntityId, signature::Signature, typeIdMaker::TypeIdMaker};

pub enum Error {}

pub trait System {
    fn New(componentIdMaker: TypeIdMaker) -> Self
    where
        Self: Sized;
    fn OnEntityEnter(&mut self, entity: EntityId);
    fn OnEntityExit(&mut self, entity: EntityId);
    fn OnEntityDestroyed(&mut self, entity: EntityId);
    fn GetSignature(&self) -> Signature;
}

pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
    systemIdMaker: TypeIdMaker,
    entityIdToSystemIndex: HashMap<EntityId, HashSet<usize>>,
}

impl SystemManager {
    pub fn New() -> SystemManager {
        SystemManager {
            systems: Vec::new(),
            systemIdMaker: TypeIdMaker::New(),
            entityIdToSystemIndex: HashMap::new(),
        }
    }

    pub fn RegisterSystem<T: System + 'static>(&mut self, system: T) {
        assert!(!self.systemIdMaker.HasTypeBeenRegistered::<T>());
        self.systemIdMaker.RegisterType::<T>();
        assert!(self.systemIdMaker.GetTypeId::<T>() == self.systems.len());

        self.systems.push(Box::new(system));
    }

    pub fn GetSystemOfType<T: System + 'static>(&mut self) -> &mut dyn System {
        assert!(self.systemIdMaker.HasTypeBeenRegistered::<T>());

        let index = self.systemIdMaker.GetTypeId::<T>();
        self.systems[index].as_mut()
    }

    pub fn OnEntityDestroyed(&mut self, entityId: EntityId, entitySignature: Signature) {
        for (idx, system) in self.systems.iter_mut().enumerate() {
            let systemIndices = self.entityIdToSystemIndex.get(&entityId).unwrap();
            let belongsToSystem = systemIndices.contains(&idx);

            let systemSignature = system.GetSignature();
            if systemSignature == entitySignature {
                assert!(belongsToSystem);
                system.OnEntityDestroyed(entityId);
            }
        }

        self.entityIdToSystemIndex.remove(&entityId);
    }

    pub fn OnEntitySignatureChanged(&mut self, entityId: EntityId, entitySignature: Signature) {
        for (idx, system) in self.systems.iter_mut().enumerate() {
            let systemIndices = self.entityIdToSystemIndex.get(&entityId).unwrap();
            let belongsToSystem = systemIndices.contains(&idx);

            let systemSignature = system.GetSignature();

            if systemSignature == entitySignature && !belongsToSystem {
                system.OnEntityEnter(entityId);

                self.entityIdToSystemIndex
                    .get_mut(&entityId)
                    .unwrap()
                    .insert(idx);
            }

            if systemSignature != entitySignature && belongsToSystem {
                system.OnEntityExit(entityId);

                self.entityIdToSystemIndex
                    .get_mut(&entityId)
                    .unwrap()
                    .remove(&idx);
            }
        }
    }

    pub fn OnEntityCreated(&mut self, entityId: EntityId, entitySignature: Signature) {
        if !self.entityIdToSystemIndex.contains_key(&entityId) {
            self.entityIdToSystemIndex.insert(entityId, HashSet::new());
        }
        self.OnEntitySignatureChanged(entityId, entitySignature);
    }
}
