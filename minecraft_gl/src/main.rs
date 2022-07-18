
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


pub extern crate gl;
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

struct Test<'a> {
    data: u32,
    dataRef: &'a u32,
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let app = Core::application::Application::New();
    app.Run();

//     let pos = nalgebra::Vector3::new(1, 5, 6);
//     let texID = 25;
//     let a = 0;
//     let b = 1;
//     let faceID = 2;
//     println!("{:08b}, {:08b}, {:08b}", 4, 0x4, 0b1111);
//     println!("{:08b}", pos.x & 0b1111);
//    let dat: u32 =  ( (pos.x & 0b1111) | ((pos.y & 0b1111) << 4) | ((pos.z & 0b11111111) << 8)) as u32; //|  (texID as i32) >> 16 | (((a * 2 + b) as i32) >> 24) & 4 | (faceID as i32 & 8) >> 26 ) as u32 ;
//    println!("{:032b}", dat);
//    test(dat, (0f32, 0f32), 2);
   
}

fn test(Data: u32, chunk_pos: (f32, f32), atlas_cols: u32){
    {
    let x = (Data & 0xF) as f32 + chunk_pos.0 * 16.0;
    let y = ( (Data >> 4u32) & 0b1111 ) as f32 + chunk_pos.1 * 16.0;
    let z = ( (Data >> 8u32) & 0b11111111 ) as f32;

    let texID = (Data >> 16u32) & 255u32; //8 bits
    let quadID = (Data >> 24u32) & 4u32; //2 bits
    let faceID = ((Data >> 26u32) & 8u32) as f32; //3 bits

    let row = (texID as u32 / atlas_cols as u32) as f32;
    let col = (texID %  atlas_cols as u32) as f32;

    println!("x {}, y {}, z {}, texID {}, quadID {}, faceID {}, row {}, col {}", x ,y ,z, texID, quadID, faceID, row, col);

    // vec2 top_left_uv = vec2(col * sprite_dimensions.x, row * sprite_dimensions.y);
    // top_left_uv = (top_left_uv + offsets[quadID] * sprite_dimensions) / texSize;
    // top_left_uv.y = 1.0 - top_left_uv.y;
    // fuvs = top_left_uv;
    }
}




