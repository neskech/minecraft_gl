
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

use std::sync::Mutex;

use rayon::{prelude::{IntoParallelRefIterator, ParallelIterator}, ThreadPoolBuilder};




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

struct Test{
    data: std::sync::Arc<Mutex<Vec<u32>>>
}

const BATCH: usize = 1;
const MAX: usize = 5;
impl Test{
    fn new() -> Self {
        Self { data: std::sync::Arc::new(Mutex::new((0..10).collect())) }
    }
    fn rayonTest(&mut self){
    
        rayon::scope(|s|{
            let i: usize = 0;
            while i < MAX {

                for b in 0..BATCH{
                    let clone = self.data.clone();
                    s.spawn( move |_|{
                        println!("{}", clone.lock().unwrap().remove(i));
                    });

                    let a = i + 1;
                    if a >= MAX {
                        break;
                    }
                }
            }
        });

 
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let app = Core::application::Application::New();
    app.Run();


 




   
}





