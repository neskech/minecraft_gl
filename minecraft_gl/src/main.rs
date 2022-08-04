
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

struct Wrapper<'a>(Test<'a>);
struct Test<'a> {
    data: Vec<u32>,
    dataRef: Vec<&'a u32>,
}

impl<'a> Test<'a> {
    pub fn New() -> Self {
        Self { data: Vec::new(), dataRef: Vec::new() }
    }

    pub fn AddChunk(&mut self){
        self.data.push(0u32);
    }

    pub fn AddRef(&'a mut self){
        if self.data.len() > 0 {
            self.dataRef.push(&self.data[0]);
        }
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let app = Core::application::Application::New();
    app.Run();

    let mut a = 10;
    let aa: *mut i32 = &mut a;
    let bb: *mut i32 = &mut a;


   
}





