
#![allow(non_snake_case)] 
#![allow(dead_code)]
#![feature(trace_macros)]
#![feature(core_intrinsics)]
#![feature(concat_idents)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(const_type_id)]

#[macro_use]
pub extern crate glium;
pub extern crate image;
pub extern crate nalgebra;

mod Core;
mod Event;
mod Util;
mod Scene;
mod Ecs;
mod World;
mod Renderer;


fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let app = Core::application::Application::New();
    app.Run();
   
}





