
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

    TextureAtlas: TextureAtlas
}

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
}


//TODO change all the errors to be Result<_, Str&> to avoid heap allcoation
impl SpriteRenderer{
    pub fn New(resourceManager: &mut ResourceManager, atlas: TextureAtlas, device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) -> Self {
        let path = "./minecraft_gl/assets/shaders/triangle.glsl";
        let shader = resourceManager.GetShader(path, device);
      

        let mut s = Self {
   
            TextureAtlas: atlas,
        };

        s.Init();
        s
    }

    pub fn Init(&mut self){
   

        
        
    }

    pub fn Render(&self, camera: &Camera, pass: &wgpu::RenderPass){


      
    }
}