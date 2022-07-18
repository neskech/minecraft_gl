#![allow(non_snake_case)] //I need to stop changing my naming convenctions ):
use std::any::Any;

//used to create trait objects out of components
pub trait CompObj{
    fn AsAny(&self) -> &dyn Any;
    fn AsAnyMut(&mut self) -> &mut dyn Any;
}
