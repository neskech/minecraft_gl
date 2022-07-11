use std::{intrinsics::variant_count};
use super::component::Component;
use super::component::GetInnerComponent;
use super::component::GetInnerComponentMut;
use proc_macro::CompObj;


const INVALID_ID: usize = usize::MAX;

#[derive(Clone)]
pub struct Entity{
    ID: usize
}

const PAGE_SIZE: usize = 100;
pub struct Pool{
    EntityList: Vec<Entity>,
    IndexList: Vec<Option<Vec<usize>>>,
    ComponentList: Vec<super::component::ComponentType>, //array of bytes representing components
}

impl Pool{
    pub fn New() -> Self{
        Self { 
            EntityList: Vec::new(),
            IndexList: Vec::new(), 
            ComponentList: Vec::new(),
        }
    }

    pub fn AddComponent<T: Component + Component<Args = Args> + Default, Args>(&mut self, entity: Entity, compArgs: Args){
        //assert that the entity doesn't already have a component of this type
        assert!(!self.HasComponent(entity.clone()), "Error! Entity already has component you're trying to add!");

        let pageIDX = entity.ID / PAGE_SIZE;
        if pageIDX >= self.IndexList.len() {
            self.IndexList.resize(pageIDX + 1, None);
        }

        self.IndexList[pageIDX] = Some(Vec::new());
        if let Some(vec) = &mut self.IndexList[pageIDX] {
            vec.reserve(PAGE_SIZE);
            vec[entity.ID % PAGE_SIZE] = self.EntityList.len();
        }   

        self.EntityList.push(entity);
        self.ComponentList.push(T::New(compArgs));
        
    }

    pub fn HasComponent(&self, entity: Entity) -> bool{
        let pageIDX = entity.ID / PAGE_SIZE;
        if  pageIDX >= self.IndexList.len() { 
            return false; 
        }

        if let Some(vec) = &self.IndexList[pageIDX] {
           return vec[entity.ID % PAGE_SIZE] != INVALID_ID;
        }
        false
    }

    pub fn RemoveComponent(&mut self, entity: Entity){
        //assert that the entity does indeed have this component type
        assert!(self.HasComponent(entity.clone()), "Error! Entity doesn't have component you're trying to remove!");

        let pageIDX = entity.ID / PAGE_SIZE;

        let mut idx: usize = 0;
        if let Some(vec) = &mut self.IndexList[pageIDX] {
            idx = vec[entity.ID % PAGE_SIZE];
            vec[entity.ID % PAGE_SIZE] = INVALID_ID;
        }

        self.EntityList.remove(idx);
        self.ComponentList.remove(idx);

    }

    pub fn GetComponent<T: Component + CompObj + 'static>(&self, entity: Entity) -> & T{
        let mut idx: usize = 0;
        let pageIDX = entity.ID / PAGE_SIZE;
        if let Some(vec) = & self.IndexList[pageIDX] {
            idx = vec[entity.ID % PAGE_SIZE];
        }
        GetInnerComponent::<T>(self.ComponentList.get(idx).unwrap())
    }

    pub fn GetComponentMut<T: Component + CompObj + 'static>(&mut self, entity: Entity) -> &mut T{
        let mut idx: usize = 0;
        let pageIDX = entity.ID / PAGE_SIZE;
        if let Some(vec) = &mut self.IndexList[pageIDX] {
            idx = vec[entity.ID % PAGE_SIZE];
        }
        GetInnerComponentMut::<T>(self.ComponentList.get_mut(idx).unwrap())
    }
}

pub struct Registry{
    Pools: Vec<Pool>,
}

impl Registry{
    pub fn New() -> Self{
        let mut vec: Vec<Pool> = Vec::new();
        //reserve num components
        vec.reserve(variant_count::<super::component::ComponentType>());

        Self {
            Pools: vec,
        }
    }

    pub fn AddComponent<T: Component + Component<Args = Args> + Default + 'static, Args>(&mut self, entity: Entity, compArgs: Args){
        let compID = super::component::Generic::<T>();
        //compID should never be >= pools.len()
        self.Pools[compID].AddComponent::<T, Args>(entity, compArgs);    
    }
    
    pub fn RemoveComponent<T: Component + CompObj + 'static>(&mut self, entity: Entity){
        let compID = super::component::Generic::<T>();
        self.Pools[compID].RemoveComponent(entity);
    }

    pub fn HasComponent<T: Component + CompObj + 'static>(&mut self, entity: Entity) -> bool{
        let compID = super::component::Generic::<T>();
        return self.Pools[compID].HasComponent(entity);
    }

    pub fn GetComponent<T: Component + CompObj + 'static>(&mut self, entity: Entity) -> &T{
        let compID = super::component::Generic::<T>();
        return self.Pools[compID].GetComponent::<T>(entity);
    }

    pub fn GetComponentMut<T: Component + CompObj + 'static>(&mut self, entity: Entity) -> &mut T{
        let compID = super::component::Generic::<T>();
        return self.Pools[compID].GetComponentMut::<T>(entity);
    }


}


