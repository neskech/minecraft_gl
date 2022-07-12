
#![allow(dead_code)]
#![allow(non_snake_case)] //I need to stop changing my naming convenctions ):
// #![feature(specialization)]
#![feature(core_intrinsics)]
#![feature(concat_idents)]
#![feature(const_type_id)]


pub extern crate gl;
pub extern crate image;
pub extern crate nalgebra;

mod Core;
mod Event;
mod OpenGL;
mod Util;
mod Scene;
mod Ecs;
mod World;

fn main() {
    // let app = Core::application::Application::New((400, 400));
    // app.Run();

}




