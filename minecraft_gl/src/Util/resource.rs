use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;
use std::rc::Rc;

use image::{self, DynamicImage, GenericImageView};
use image::io::Reader as ImageReader;


pub struct ResourceManager{
    Shaders: HashMap<&'static str, Rc<wgpu::ShaderModule>>,
    Textures: HashMap<&'static str, Rc<wgpu::Texture>>,
}

impl ResourceManager{
    pub fn New() -> Self {
        Self {
            Shaders: HashMap::new(),
            Textures: HashMap::new()
        }
    }


    pub fn GetShader(&mut self, path: &'static str, device: &wgpu::Device) -> Result<Rc<wgpu::ShaderModule>, String> {
        if self.Shaders.contains_key(path) {
            return Ok(Rc::clone(&self.Shaders[path]));
        }
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(std::fs::read_to_string(path).
                map_err(|e| format!("Error! Resource manager could not read shader of path {}. The error:\n{}", path, e.to_string()))?.into()),
        });
        self.Shaders.insert(path, Rc::new(shader));
        Ok(Rc::clone(&self.Shaders[path]))
    }

    pub fn GetTexture(&mut self, path: &'static str, mipMapLevels: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Rc<wgpu::Texture>, String> {
        if self.Textures.contains_key(path) {
            return Ok(Rc::clone(&self.Textures[path]));
        }
        self.Textures.insert(path, Rc::new(GetTextureFromPath(path, mipMapLevels, device, queue).
        map_err(|e| format!("Error! Resource manager could not read texture of path {}. The error:\n{}", path, e.to_string()))?));
        Ok(Rc::clone(&self.Textures[path]))
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

pub fn GetImageOrNull(path: &str) -> Result<DynamicImage, DynamicImage> {
    if Path::new(path).exists() {
       return Ok(GetImageFromPath(path).unwrap());
    }

    Err(GetImageFromPath("./minecraft_gl/assets/data/block/img/nullTexture.png").unwrap())
}

pub fn GetTextureFromPath(path: &str, mipMapLevels: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<wgpu::Texture, String> {
    let img = ImageReader::open(path)
    .map_err(|e| format!("Error! Could not read image from path of: {}. The error:\n{}", path, e.to_string()))?;

    let decoded = img.decode()
    .map_err(|e| format!("Error! Could not decode image from path of: {}. The error:\n{}", path, e.to_string()))?;

    Ok(GetTextureFromImage(&decoded, mipMapLevels, device, queue))
}  

pub fn GetTextureFromImage(image: &DynamicImage, mipMapLevels: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture{
    let dims = image.dimensions();
   
    let texture_size = wgpu::Extent3d {
        width: dims.0,
        height: dims.1,
        depth_or_array_layers: 1,
    };

    let diffuse_texture = device.create_texture(
        &wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size,
            mip_level_count: mipMapLevels, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
        }
    );

    queue.write_texture(
        // Tells wgpu where to copy the pixel data
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        // The actual pixel data
        &image.as_bytes(),
        // The layout of the texture
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: std::num::NonZeroU32::new(4 * dims.0),
            rows_per_image: std::num::NonZeroU32::new(dims.1),
        },
        texture_size,
    );

    diffuse_texture
}