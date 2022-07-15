use std::collections::HashMap;
use std::rc::Rc;

use image::{self, DynamicImage};
use image::io::Reader as ImageReader;

use crate::OpenGL::shader::Shader;
use crate::OpenGL::texture::Texture;

pub struct ResourceManager{
    Shaders: HashMap<&'static str, Rc<Shader>>,
    Textures: HashMap<&'static str, Rc<Texture>>,
}

impl ResourceManager{
    pub fn New() -> Self {
        Self {
            Shaders: HashMap::new(),
            Textures: HashMap::new()
        }
    }

    pub fn HasShader(&self, path: &str) -> bool {
        self.Shaders.contains_key(path)
    }

    pub fn HasTexture(&self, path: &str) -> bool {
        self.Textures.contains_key(path)
    }

    pub fn InsertShader(&mut self, path: &'static str, shader: Shader) -> Rc<Shader>{
        self.Shaders.insert(path, Rc::new(shader));
        Rc::clone(&self.Shaders[path])
    }

    pub fn InsertTexture(&mut self, path: &'static str, texture: Texture) -> Rc<Texture>{
        self.Textures.insert(path, Rc::new(texture));
        Rc::clone(&self.Textures[path])
    }

    pub fn GetShader(&self, path: &str) -> Option<Rc<Shader>> {
        if self.Shaders.contains_key(path) {
            return Some(Rc::clone(&self.Shaders[path]));
        }
        None
    }

    pub fn GetTexture(&self, path: &str) -> Option<Rc<Texture>> {
        if self.Textures.contains_key(path) {
            return Some(Rc::clone(&self.Textures[path]));
        }
        None
    }
}



pub fn GetImageFromPath(path: &str) -> Result<DynamicImage, String> {

    let img = ImageReader::open(path);
    if let Err(_) = img {
        return Err(format!("Error! Could not read texture atlas from path of: {}", path));
    }

    let decoded = img.unwrap().decode();
    if let Err(e) = decoded {
        return Err(format!("Error! Could not decode image from path of: {}. The error:\n{}", path, e.to_string()));
    }

    Ok(decoded.unwrap())
}