use std::collections::HashMap;
use std::io::BufRead;
use std::rc::Rc;

use image::{self, DynamicImage, GenericImageView};
use image::io::Reader as ImageReader;


pub struct ResourceManager{
    Shaders: HashMap<&'static str, Rc<glium::Program>>,
    Textures: HashMap<&'static str, Rc<glium::texture::SrgbTexture2d>>,
}

impl ResourceManager{
    pub fn New() -> Self {
        Self {
            Shaders: HashMap::new(),
            Textures: HashMap::new()
        }
    }


    pub fn GetShader(&mut self, path: &'static str, display: &glium::Display) -> Rc<glium::Program> {
        if self.Shaders.contains_key(path) {
            return Rc::clone(&self.Shaders[path]);
        }
        self.Shaders.insert(path, Rc::new(GetShaderFromPath(path, display).unwrap()));
        Rc::clone(&self.Shaders[path])
    }

    pub fn GetTexture(&mut self, path: &'static str, display: &glium::Display) -> Rc<glium::texture::SrgbTexture2d> {
        if self.Textures.contains_key(path) {
            return Rc::clone(&self.Textures[path]);
        }
        self.Textures.insert(path, Rc::new(GetTextureFromPath(path, display).unwrap()));
        Rc::clone(&self.Textures[path])
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

pub fn GetShaderFromPath(path: &str, display: &glium::Display) -> Result<glium::Program, String>{
    let file = std::fs::File::open(path)
    .map_err(|e| format!("Could not open file of path {} in 'GetShaderFromPath' function", path))?;
    
    let fileLines = std::io::BufReader::new(file).lines();

    let mut vertex = String::from("");
    let mut fragment = String::from("");

    let mut shaderID = 0;
    for line in fileLines {
        let content = line.unwrap();

        if content.contains("#type vertex"){
            shaderID = 0;
            continue;
        }
        else if content.contains("#type fragment"){
            shaderID = 1;
            continue;
        }
        
        if shaderID == 0 {
            vertex = format!("{}\n{}", vertex, content);
        }
        else {
            fragment = format!("{}\n{}", fragment, content);
        }
    }

    glium::Program::from_source(display, &vertex, &fragment, None)
    .map_err(|e| format!("Shader program creation failed. The error:\n{}", e.to_string()))
    
}

pub fn GetTextureFromPath(path: &str, display: &glium::Display) -> Result<glium::texture::SrgbTexture2d, String> {
    let img = ImageReader::open(path)
    .map_err(|e| format!("Error! Could not read image from path of: {}. The error:\n{}", path, e.to_string()))?;

    let decoded = img.decode()
    .map_err(|e| format!("Error! Could not decode image from path of: {}. The error:\n{}", path, e.to_string()))?;

    let dims = decoded.dimensions();
    let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&decoded.as_bytes(), dims);

    glium::texture::SrgbTexture2d::new(display, raw)
    .map_err(|e| format!("Error! Could not create texture from image of path {}. The error:\n{}", path, e.to_string()))
}  

pub fn GetTextureFromImage(image: &DynamicImage, display: &glium::Display) -> Result<glium::texture::SrgbTexture2d, String>{
    let dims = image.dimensions();
    let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.as_bytes(), dims);
    glium::texture::SrgbTexture2d::new(display, raw)
    .map_err(|e| format!("Error! Could not create texture from the given image. The error:\n{}", e.to_string()))

}