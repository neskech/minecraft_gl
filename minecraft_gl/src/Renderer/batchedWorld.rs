use std::rc::Rc;

use glfw::WindowEvent;

use crate::OpenGL::buffer::{VertexBuffer, IndexBuffer};
use crate::OpenGL::shader::Shader;
use crate::OpenGL::texture::Texture;
use crate::OpenGL::vao::VAO;


pub struct Vertex{
    Pos: (f32, f32, f32),
    TexID: u8,
    FaceID: u8,
}

struct WorldRenderer{
    VertexBuffer: VertexBuffer,
    IndexBuffer: IndexBuffer,
    VAO: VAO,
    Shader: Rc<Shader>,
    TextureAtlas: Rc<Texture>
}

impl WorldRenderer{
    // pub fn New() -> Self {
    //     Self {
    //         VertexBuffer: VertexBuffer::New(),
    //         IndexBuffer: IndexBuffer::New(),
    //         VAO: VAO::New(),
    //         Shader: ,
    //         TextureAtlas: ,
    //     }
    // }

    pub fn Init(&mut self){

    }

    pub fn Render(&self){

    }

    pub fn OnEvent(&mut self, event: WindowEvent){

    }

    pub fn OnRenderListUpdate(&mut self, event: WindowEvent){
        
    }

    pub fn OnChunkRemesh(&mut self, event: WindowEvent){
        
    }
}