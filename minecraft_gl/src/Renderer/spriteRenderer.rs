use std::rc::Rc;
use crate::OpenGL::texture::Texture;
use crate::OpenGL::texture::TextureParams;
use crate::OpenGL::buffer::{VertexBuffer, IndexBuffer};
use crate::OpenGL::shader::Shader;
use crate::OpenGL::vao::VAO;
use crate::Util::atlas::TextureAtlas;
use crate::Util::resource::ResourceManager;
use crate::World::item::ItemRegistry;

const BLOCK_TEXTURE_RESOLUTION: u32 = 16;

pub struct Vertex{
    Pos: u16,
    TexID: u8,
    FaceID: u8,
}

pub struct SpriteRenderer{
    VertexBuffer: VertexBuffer,
    IndexBuffer: IndexBuffer,
    VAO: VAO,
    Shader: Rc<Shader>,
    TextureAtlas: TextureAtlas
}

//TODO change all the errors to be Result<_, Str&> to avoid heap allcoation
impl SpriteRenderer{
    pub fn New(resourceManager: &mut ResourceManager, itemRegistry: &ItemRegistry) -> Result<Self, String> {
        let atlasTexture = Texture::New().SetTextureParams(TextureParams {
            WrapX: gl::CLAMP_TO_EDGE,
            WrapY: gl::CLAMP_TO_EDGE,
            MinFilter: gl::NEAREST,
            MagFilter: gl::NEAREST,
            MipmapLevels: Some(2),
        });

        let atlas = match itemRegistry.GenerateAtlas(atlasTexture, BLOCK_TEXTURE_RESOLUTION) {
            Ok(val) => val,
            Err(msg) => {
                return Err(format!("Error! World renderer creation failed due to block atlas creation. The error:\n{}.", msg));
            }
        };

        let path = "../../assets/shaders/world.glsl";
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

        s.Init();
        Ok(s)
    }

    pub fn Init(&mut self){
        self.VAO.Bind();
        self.IndexBuffer.Bind();

  
        self.IndexBuffer.UnBind();
        self.VAO.UnBind();
    }

    pub fn Render(&self){
        self.VAO.Bind();
        self.VertexBuffer.Bind();

        self.Shader.Activate();
    
        self.Shader.DeActivate();

        self.VAO.UnBind();
        self.VertexBuffer.UnBind();
    }
}