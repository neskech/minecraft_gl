
#![allow(dead_code)]
#![allow(non_snake_case)] //I need to stop changing my naming convenctions ):
// #![feature(specialization)]
#![feature(backtrace)]
#![feature(trace_macros)]
#![feature(core_intrinsics)]
#![feature(core_c_str)]
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





