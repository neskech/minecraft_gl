
use image::*;
use image::io::Reader as ImageReader;

use crate::OpenGL::texture::Texture;


pub struct TextureAtlas{
    pub Texture: Texture,
    pub Image: DynamicImage,
    pub Rows: u32,
    pub Columns: u32,
    pub CellHeight: u32,
    pub CellWidth: u32,
    pub format: i32,
}

pub struct CubeMapAtlas{
    pub Texture: Texture,
    pub Image: DynamicImage,
    pub Rows: u32,
    pub Columns: u32,
    pub CellHeight: u32,
    pub CellWidth: u32,
    pub FaceHeight: u32,
    pub FaceWidth: u32,
    pub format: u32,
}

pub enum CubeMapFace{
    Top,
    MiddleLeft,
    Middle,
    MiddleRight,
    Bottom
}


impl TextureAtlas{
    pub fn New(path: &str, texture: Texture, rows: u32, cols: u32) -> Result<TextureAtlas, String>{
        //Assume that the texture passed already has its texture parameters set
        let img = ImageReader::open(path);
        if let Err(_) = img {
            return Err(format!("Error! Could not read texture atlas from path of: {}", path));
        }

        let decoded = img.unwrap().decode();
        if let Err(_) = decoded {
            return Err(format!("Error! Could not decode image from path of: {}", path));
        }

        let finalImage = decoded.unwrap();
        let channels = if finalImage.color().has_alpha(){
                                 gl::RGBA 
                            } else { 
                                 gl::RGB 
                            };
        
        let cellHeight = (finalImage.height() as f32 / rows as f32) as u32;
        let cellWidth = (finalImage.width() as f32 / cols as f32) as u32;
        Ok(Self {
            Texture: texture.SetImage(&finalImage, channels),
            Image: finalImage,
            Rows: rows,
            Columns: cols,
            CellHeight: cellHeight,
            CellWidth:  cellWidth,
            format: channels as i32
        })
    }

    pub fn FromImage(image: DynamicImage, texture: Texture, rows: u32, cols: u32, textureResolution: u32) -> Self{
        //Assume that the texture passed already has its texture parameters set
        let channels = if image.color().has_alpha(){
                            gl::RGBA 
                    } else { 
                            gl::RGB 
                    };

        Self {
            Texture: texture.SetImage(&image, channels),
            Image: image,
            Rows: rows,
            Columns: cols,
            CellHeight: textureResolution,
            CellWidth: textureResolution,
            format: channels as i32
        }
    }

    pub fn GrabSubImage(&self, row: u32, col: u32) ->  SubImage<&DynamicImage>{
        let x = col * self.CellWidth;
        let y = row * self.CellHeight;
        SubImage::new(&self.Image, x, y, self.CellWidth, self.CellHeight) 
    }


}


impl CubeMapAtlas{
    pub fn New(path: &str, texture: Texture, rows: u32, cols: u32) -> Result<CubeMapAtlas, String>{

        let img = ImageReader::open(path);
        if let Err(_) = img {
            return Err(format!("Error! Could not read texture atlas from path of: {}", path));
        }

        let decoded = img.unwrap().decode();
        if let Err(_) = decoded {
            return Err(format!("Error! Could not decode image from path of: {}", path));
        }

        let finalImage = decoded.unwrap();
        let channels = if finalImage.color().has_alpha(){
                                 gl::RGBA 
                            } else { 
                                 gl::RGB 
                            };


        let cellHeight = (finalImage.height() as f32 / rows as f32) as u32;
        let cellWidth = (finalImage.width() as f32 / cols as f32) as u32;                 
        Ok(Self {
            Texture: texture.SetImage(&finalImage, channels),
            Image: finalImage,
            Rows: rows,
            Columns: cols,
            CellHeight: cellHeight,
            CellWidth:  cellWidth,
            FaceHeight: (cellHeight as f32 / 3f32) as u32,
            FaceWidth: (cellWidth as f32 / 3f32) as u32,
            format: channels
        })
    }

    pub fn GrabSubImage(&self, row: u32, col: u32, face: &CubeMapFace) -> SubImage<&DynamicImage>{
        let (offsetX, offsetY): (u32, u32) = match face {
            CubeMapFace::Top => (self.FaceWidth, 0u32),
            CubeMapFace::MiddleLeft => (0u32, self.FaceHeight),
            CubeMapFace::Middle => (self.FaceWidth, self.FaceHeight),
            CubeMapFace::MiddleRight => (self.FaceWidth * 2u32, self.FaceHeight),
            CubeMapFace::Bottom => (self.FaceWidth, self.FaceHeight * 2u32),
        };

        let x = col * self.CellWidth + offsetX;
        let y = row * self.CellHeight + offsetY;
        SubImage::new(&self.Image, x, y, self.FaceWidth, self.FaceHeight) 
    }

}