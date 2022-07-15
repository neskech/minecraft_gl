use std::rc::Rc;

use std::ffi::CString;
use nalgebra as na;
use crate::OpenGL::texture::Texture;
use crate::OpenGL::texture::TextureParams;
use crate::Scene::camera::Camera;
use crate::World::chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Y, CHUNK_BOUNDS_Z};
use crate::OpenGL::buffer::{VertexBuffer, IndexBuffer};
use crate::OpenGL::shader::Shader;
use crate::OpenGL::vao::VAO;
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource::ResourceManager;
use crate::World::block::BlockRegistry;
use crate::World::chunk::Chunk;

const BLOCK_TEXTURE_RESOLUTION: u32 = 16;

pub struct Vertex{
   pub Data: u32
}

pub struct WorldRenderer{
    VertexBuffer: VertexBuffer,
    IndexBuffer: IndexBuffer,
    VAO: VAO,
    Shader: Rc<Shader>,
    TextureAtlas: TextureAtlas
}

//TODO change all the errors to be Result<_, Str&> to avoid heap allcoation
impl WorldRenderer{
    pub fn New(resourceManager: &mut ResourceManager, blockRegistry: &BlockRegistry) -> Result<Self, String> {
        let atlasTexture = Texture::New().SetTextureParams(TextureParams {
            WrapX: gl::CLAMP_TO_EDGE,
            WrapY: gl::CLAMP_TO_EDGE,
            MinFilter: gl::NEAREST,
            MagFilter: gl::NEAREST,
            MipmapLevels: Some(2),
        });

        let atlas = match blockRegistry.GenerateAtlas(atlasTexture, BLOCK_TEXTURE_RESOLUTION) {
            Ok(val) => val,
            Err(msg) => {
                return Err(format!("Error! World renderer creation failed due to block atlas creation. The error:\n{}.", msg));
            }
        };

        let path = "./minecraft_gl/assets/shaders/world.glsl";
        let shader = match resourceManager.GetShader(path) {
             Some(val) => val,
             None => resourceManager.InsertShader(path, Shader::New(path))
        };
      
        let mut s = Self {
            VertexBuffer: VertexBuffer::New(),
            IndexBuffer: IndexBuffer::New(),
            VAO: VAO::New(),
            Shader: shader,
            TextureAtlas: atlas,
        };

        //validate that the size of a chunk is enough to cover with 2 bytes
        let size = CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z;
        if size > u16::MAX as u32 {
            return Err(format!("Error! Cannot create world renderer because the size of a chunk ({} blocks) is to large for the 2 byte unsigned int ceiling ({})", size, u16::MAX));
        }

        s.Init();
        Ok(s)
    }

    pub fn Init(&mut self){
         self.VAO.Bind();
         println!("HEHEHEHHEHE {}", self.IndexBuffer.IsValid());
         self.IndexBuffer.Bind();
         CheckGLError();
        self.VertexBuffer.Bind();
        CheckGLError();
        let vertSizeBytes = 4; //4 bytes
        //1D local block index (position) -> 2 bytes
        //texture ID (range: 0-u8::limit, 0-255) -> 1 byte //TODO get validation that the num textures doesnt exceed u8::limit
        //face ID (range :0-6) -> 3 bits 
        println!("Before");
        //CheckGLError();
        //self.VAO.AddAtribute::<u32>(1, vertSizeBytes, None);
       // CheckGLError();
        print!("after");

        let maxNumQuads: usize = (CHUNK_BOUNDS_X * CHUNK_BOUNDS_Y * CHUNK_BOUNDS_Z * 4) as usize;
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
        
        self.IndexBuffer.BufferData(&indices, gl::STATIC_DRAW);
        println!("after index buffer buffer");
        CheckGLError();
        self.VertexBuffer.UnBind();
        self.IndexBuffer.UnBind();
        self.VAO.UnBind();
    }

    pub fn Render(&self, chunks: &Vec<Chunk>, renderList: &Vec<usize>, camera: &Camera){
        self.VAO.Bind();
        self.VertexBuffer.Bind();
        CheckGLError();
        println!("after vbo bind!!!!");
        self.Shader.Activate();
        self.TextureAtlas.Texture.Bind();
        CheckGLError();
        println!("after bind and atvitate");
        self.Shader.UploadMatrix4x4(camera.GetProjectionMatrix(), CString::new("proj").unwrap());
        self.Shader.UploadMatrix4x4(camera.GetViewMatrix(), CString::new("view").unwrap());
        self.Shader.UploadVec2(na::Vector2::new(16 as f32, 16 as f32), CString::new("sprite_dimensions").unwrap());
        self.Shader.UploadFloat(self.TextureAtlas.Columns as f32, CString::new("atlas_cols").unwrap());
        self.Shader.UploadVec2(na::Vector2::new((self.TextureAtlas.CellWidth * self.TextureAtlas.Columns) as f32, (self.TextureAtlas.CellHeight * self.TextureAtlas.Rows) as f32), CString::new("texSize").unwrap());
        CheckGLError();
        for idx in renderList {
            let chunk = &chunks[*idx];

            self.Shader.UploadVec2(na::Vector2::new(chunk.ChunkPosition.0 as f32, chunk.ChunkPosition.1 as f32), CString::new("chunk_pos").unwrap());
            CheckGLError();
            println!("After vec 2!");
            self.VertexBuffer.BufferData::<Vertex>(&chunk.Mesh, gl::STATIC_DRAW);
            println!("After buffer!");
            CheckGLError();
            unsafe {
                gl::DrawElements(gl::TRIANGLES, ((chunk.Mesh.len() / 4) * 6) as i32, gl::UNSIGNED_INT, std::ptr::null());
            }
        }
        self.TextureAtlas.Texture.UnBind();
        self.Shader.DeActivate();

        self.VAO.UnBind();
        self.VertexBuffer.UnBind();
    }
}

fn CheckGLError()
{
    unsafe {
        let mut err: gl::types::GLenum = gl::GetError();
        while err != gl::NO_ERROR{
            println!("{}", err);
            err = gl::GetError();
        }  
    }
}