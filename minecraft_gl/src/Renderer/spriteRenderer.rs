use std::rc::Rc;
use glium::Surface;

use crate::Scene::camera::Camera;
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource::ResourceManager;

const BLOCK_TEXTURE_RESOLUTION: u32 = 16;

// #[derive(Clone, Copy)]
// pub struct Vertex{
//     Pos: u16,
//     TexID: u8,
//     FaceID: u8,
// }

pub struct SpriteRenderer{
    VertexBuffer: glium::VertexBuffer<Vertex>,
    IndexBuffer: glium::IndexBuffer<u32>,
    Shader: Rc<glium::Program>,
    TextureAtlas: TextureAtlas
}

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
}

implement_vertex!(Vertex, pos);

//TODO change all the errors to be Result<_, Str&> to avoid heap allcoation
impl SpriteRenderer{
    pub fn New(resourceManager: &mut ResourceManager, atlas: TextureAtlas, display: &glium::Display) -> Self {
        let path = "./minecraft_gl/assets/shaders/triangle.glsl";
        let shader = resourceManager.GetShader(path, display);
      

        let mut s = Self {
            VertexBuffer: glium::VertexBuffer::empty_dynamic(display, 4)
            .expect("Sprite Renderer's Vertex buffer creation failed!"),
            IndexBuffer: glium::IndexBuffer::empty(display, glium::index::PrimitiveType::TrianglesList, 3)
            .expect("Sprite Renderer's Index buffer creation failed!"),
            Shader: shader,
            TextureAtlas: atlas,
        };

        s.Init();
        s
    }

    pub fn Init(&mut self){
        let data = vec![0, 1, 2];
        self.IndexBuffer.write(&data);
        let scale = 2f32;
        let vertex1 = Vertex { pos: [-0.5 * scale, -0.5 * scale, -0.7 * scale] };
        let vertex2 = Vertex { pos: [ 0.0 * scale,  -0.5 * scale, -0.7 * scale] };
        let vertex3 = Vertex { pos: [ 0.5 * scale, -0.25 * scale, -0.7 * scale] };
        let shape = vec![vertex1, vertex2, vertex3];

        let mapping = self.VertexBuffer.map().as_mut_ptr();
        for i in 0..3 {
            unsafe { *mapping.add(i) = shape[i]; }
        }

        
        //self.VertexBuffer.write(&shape);
    }

    pub fn Render(&self, camera: &Camera, target: &mut glium::Frame){


        let uniforms = uniform! {
            proj: camera.GetProjectionMatrix(),
            view: camera.GetViewMatrix(),
        };

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        target.draw(&self.VertexBuffer, &indices, &self.Shader, &uniforms,
            &Default::default()).unwrap();

    }
}