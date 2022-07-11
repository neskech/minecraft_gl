

use std::ffi::c_void;

use gl::types::*;
use image::*;
use image::io::Reader as ImageReader;

use crate::Util::atlas::{CubeMapAtlas, CubeMapFace};

struct CubeMapParams{
    WrapX: GLint,
    WrapY: GLint,
    WrapZ: GLint,
    MinFilter: GLint,
    MagFilter: GLint,
    MipmapLevels: Option<i32>
}

struct CubeMap{
    ID: GLuint
}

impl CubeMap{
    pub fn New() -> Self {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id); }
        Self { ID: id }
    }

    //builder pattern
    pub fn SetTextureParams(self, params: CubeMapParams) -> Self {
        unsafe {
            self.Bind(); 
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, params.WrapX);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, params.WrapY);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, params.WrapZ);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, params.MinFilter);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, params.MagFilter);

            if let Some(mipMapLevels) = params.MipmapLevels {
                gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAX_LEVEL, mipMapLevels);
            }
            self.UnBind();
        }
        self
    }

    //builder pattern -- meant to be called last
    pub fn SetTextureFromPaths(self, paths: &[&str]) -> Result<Self, String> {

        for i in 0..paths.len() {

            let img = ImageReader::open(paths[i]);
            if let Err(_) = img {
                return Err(format!("Error! Could not read image from path of: {}", paths[i]));
            }

            let decoded = img.unwrap().decode();
            if let Err(_) = decoded {
                return Err(format!("Error! Could not decode image from path of: {}", paths[i]));
            }
            let finalImage = decoded.unwrap();
            let channels = if finalImage.color().has_alpha(){
                                    gl::RGBA 
                                } else { 
                                    gl::RGB 
                                };

            unsafe {
                self.Bind();
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    channels as i32,
                    finalImage.width() as i32,
                    finalImage.height() as i32,
                    0,
                    channels,
                    gl::UNSIGNED_BYTE,
                    finalImage.as_bytes() as *const _ as *const c_void
                );
                self.UnBind();
            }
        }

        Ok(self)
    }

    pub fn setTextureFromAtlas(self, atlas: &CubeMapAtlas, row: u32, col: u32) -> Result<Self, String>{
        let faces = 
            [CubeMapFace::Top, CubeMapFace::Top, CubeMapFace::MiddleLeft, CubeMapFace::Middle, 
            CubeMapFace::MiddleRight, CubeMapFace::Bottom];

        for i in 0..6 {

            unsafe {
                self.Bind();
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i,
                    0,
                    atlas.format as i32,
                    atlas.FaceWidth as i32,
                    atlas.FaceHeight as i32,
                    0,
                    atlas.format,
                    gl::UNSIGNED_BYTE,
                    atlas.GrabSubImage(row, col, &faces[i as usize]).to_image().as_bytes() as *const _ as *const c_void
                );
                self.UnBind();
            }
        }

        Ok(self)
    }


    pub fn Bind(&self){
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.ID) };
    }

    pub fn UnBind(&self){
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) };
    }

    pub fn Destroy(&self){
        unsafe { gl::DeleteTextures(1, &self.ID) };
    }
}

impl Drop for CubeMap{
    fn drop(&mut self) {
        self.Destroy();
    }
}