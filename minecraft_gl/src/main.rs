
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

use Core::application::Application;

pub extern crate winit;

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
    Application::Run();


   
}






