
use std::rc::Rc;
use glium::Surface;
use glium::uniforms::{MinifySamplerFilter, MagnifySamplerFilter};
use image::GenericImageView;
use crate::Scene::camera::Camera;
use crate::World::chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Y, CHUNK_BOUNDS_Z};
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource::ResourceManager;
use crate::World::chunk::Chunk;

pub const BLOCK_TEXTURE_RESOLUTION: u32 = 16;

#[derive(Clone, Copy, Debug)]
pub struct Vertex{
   pub Data: u32
}

pub struct WorldRenderer{
    VertexBuffer: glium::VertexBuffer<Vertex>,
    IndexBuffer: glium::IndexBuffer<u32>,
    Shader: Rc<glium::Program>,
    TextureAtlas: TextureAtlas
}

//TODO change all the errors to be Result<_, Str&> to avoid heap allcoation
impl WorldRenderer{
    pub fn New(resourceManager: &mut ResourceManager, atlas: TextureAtlas, display: &glium::Display) -> Self {
        implement_vertex!(Vertex, Data);

        let path = "./minecraft_gl/assets/shaders/world.glsl";
        let shader = resourceManager.GetShader(path, display);
      
        let mut s = Self {
            VertexBuffer: glium::VertexBuffer::empty_dynamic(display, (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z * 4 * 6) as usize)
            .expect("Sprite Renderer's Vertex buffer creation failed!"),
            IndexBuffer: glium::IndexBuffer::empty(display, glium::index::PrimitiveType::TrianglesList,
        (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z * 6 * 6) as usize)
            .expect("Sprite Renderer's Index buffer creation failed!"),
            Shader: shader,
            TextureAtlas: atlas,
        };

        //validate that the size of a chunk is enough to cover with 2 bytes
        let size = CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z;
        if size > u16::MAX as u32 {
            panic!("Error! Cannot create world renderer because the size of a chunk ({} blocks) is to large for the 2 byte unsigned int ceiling ({})", size, u16::MAX);
        }

        s.Init();
        s
    }

    pub fn Init(&mut self){
   


        let maxNumQuads: usize = (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z * 6) as usize;
        let mut indices: Vec<u32> = vec![0; maxNumQuads * 6];
        for i in 0..maxNumQuads {
            let c = i as u32;
            indices[0 + i * 6] = 0 + c * 4;
            indices[1 + i * 6] = 1 + c * 4;
            indices[2 + i * 6] = 3 + c * 4;
            indices[3 + i * 6] = 0 + c * 4;
            indices[4 + i * 6] = 3 + c * 4;
            indices[5 + i * 6] = 2 + c * 4;
        }
        
        self.IndexBuffer.write(indices.as_slice());
 

    }

    pub fn Render(&mut self, chunks: &Vec<Chunk>, renderList: &Vec<usize>, camera: &Camera, target: &mut glium::Frame){

        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: MinifySamplerFilter ::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        for idx in renderList {
            let chunk = &chunks[*idx];
            let dims = (self.TextureAtlas.Image.dimensions().0 as f32, self.TextureAtlas.Image.dimensions().1 as f32);
            let uniforms = uniform! {
                proj: camera.GetProjectionMatrix(),
                view: camera.GetViewMatrix(),
                sprite_dimensions: [16f32, 16f32],
                atlas_cols: self.TextureAtlas.Columns as f32,
                texSize: [dims.0, dims.1],
                chunk_pos: [chunk.Position.0 as f32, chunk.Position.1 as f32],
                atlas: glium::uniforms::Sampler(&self.TextureAtlas.Texture, behavior)
            };

           // self.VertexBuffer.write(&chunk.Mesh);
           let mapping = self.VertexBuffer.map().as_mut_ptr();
        //     for i in 0..chunk.Mesh.len() {
        //         unsafe { *mapping.add(i) = chunk.Mesh[i]; 
        //             //println!("Vert {:?}", *mapping.add(i));
        //         }
        //     }
            unsafe { mapping.copy_from(chunk.Mesh.as_ptr(), chunk.Mesh.len()); }

            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                backface_culling: glium::BackfaceCullingMode::CullingDisabled,

                .. Default::default()
            };

            target.draw(&self.VertexBuffer, &self.IndexBuffer, &self.Shader, &uniforms,
                &params).unwrap();
         
        }

    }
}
