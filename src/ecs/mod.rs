use thiserror::Error;

pub mod componentAllocator;
pub mod componentIdMaker;
pub mod componentManager;
pub mod constants;
pub mod entity;
pub mod entityManager;
pub mod layerManager;
pub mod signature;

#[derive(Error, Debug)]
pub enum Error {
    #[error(source)]
    ComponentAllocatorError(componentAllocator::Error),
}
