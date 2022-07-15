
#![allow(dead_code)]
#![allow(non_snake_case)] //I need to stop changing my naming convenctions ):
// #![feature(specialization)]
#![feature(backtrace)]
#![feature(core_intrinsics)]
#![feature(core_c_str)]
#![feature(concat_idents)]
#![feature(cstr_from_bytes_until_nul)]
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
mod Renderer;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = Core::application::Application::New((400, 400));
    app.Run();
   
}




