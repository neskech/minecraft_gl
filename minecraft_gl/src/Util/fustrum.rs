use crate::{Core::application::WINDOW_SIZE, World::chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Y, CHUNK_BOUNDS_Z}};
use nalgebra as na;


pub struct Fustrum {
    pub Planes: [na::Vector4<f32>; 6]
}

impl Fustrum {
    pub fn New(mvp: &na::Matrix4<f32>) -> Self {
       let mut self_ = Self {
           Planes: [na::Vector4::zeros(); 6]
       };

       self_.update(mvp);
       self_
    }

    pub fn update(&mut self, mvp:  &na::Matrix4<f32>){
        self.Planes[0] = na::Vector4::new(mvp[3] - mvp[0], mvp[7] - mvp[4], mvp[11] - mvp[8], mvp[15] - mvp[12]);
        self.Planes[1] = na::Vector4::new(mvp[3] + mvp[0], mvp[7] + mvp[4], mvp[11] + mvp[8], mvp[15] + mvp[12]);
        self.Planes[2] = na::Vector4::new(mvp[3] - mvp[1], mvp[7] - mvp[5], mvp[11] - mvp[9], mvp[15] - mvp[13]);
 
        self.Planes[3] = na::Vector4::new(mvp[3] - mvp[1], mvp[7] - mvp[5], mvp[11] - mvp[9], mvp[15] - mvp[13]);
        self.Planes[4] = na::Vector4::new(mvp[3] + mvp[2], mvp[7] + mvp[6], mvp[11] + mvp[10], mvp[15] + mvp[14]);
        self.Planes[5] = na::Vector4::new(mvp[3] - mvp[2], mvp[7] - mvp[6], mvp[11] - mvp[10], mvp[15] - mvp[14]);

        for i in 0..6{
            self.Planes[i] = self.Planes[i].normalize();
       }
    }
    
    pub fn CheckChunk(&self, min: &na::Vector3<f32>, max: &na::Vector3<f32>) -> bool{
        for i in 0..6 {
            let mut b = true;
            b = b && self.Planes[i][0] * min.x + self.Planes[i][1] * min.y + self.Planes[i][2] * min.z + self.Planes[i][3] <= 0_f32;
            b = b && self.Planes[i][0] * max.x + self.Planes[i][1] * min.y + self.Planes[i][2] * min.z + self.Planes[i][3] <= 0_f32;
            b = b && self.Planes[i][0] * min.x + self.Planes[i][1] * max.y + self.Planes[i][2] * min.z + self.Planes[i][3] <= 0_f32;
            b = b && self.Planes[i][0] * max.x + self.Planes[i][1] * max.y + self.Planes[i][2] * min.z + self.Planes[i][3] <= 0_f32;
            b = b && self.Planes[i][0] * min.x + self.Planes[i][1] * min.y + self.Planes[i][2] * max.z + self.Planes[i][3] <= 0_f32;
            b = b && self.Planes[i][0] * max.x + self.Planes[i][1] * min.y + self.Planes[i][2] * max.z + self.Planes[i][3] <= 0_f32;
            b = b && self.Planes[i][0] * min.x + self.Planes[i][1] * max.y + self.Planes[i][2] * max.z + self.Planes[i][3] <= 0_f32;
            b = b && self.Planes[i][0] * max.x + self.Planes[i][1] * max.y + self.Planes[i][2] * max.z + self.Planes[i][3] <= 0_f32;
            if b { return false; }
        }
        true
    }
}

