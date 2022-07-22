use crate::{Core::application::WINDOW_SIZE, World::chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Y, CHUNK_BOUNDS_Z}};
use nalgebra as na;

pub struct Plane {
    pub Normal: na::Vector3<f32>,
    pub Distance: na::Vector3<f32>, //distance to origin
}

impl Plane {
    pub fn DistanceTo(&self, point: &na::Vector3<f32>) -> f32 {
        (self.Distance - point).dot(&self.Normal)
    }

    pub fn ChunkCollidesWith(&self, chunkPos: &na::Vector3<f32>) -> bool{
        //from https://gdbooks.gitbooks.io/3dcollisions/content/Chapter2/static_aabb_plane.html
        let extents = na::Vector3::new(
             CHUNK_BOUNDS_X as f32 / 2f32,
              CHUNK_BOUNDS_Y as f32 / 2f32,
              CHUNK_BOUNDS_Z as f32 / 2f32);

        let r = extents.dot(&self.Normal.abs());
        let dist = self.DistanceTo(chunkPos);
        //f32::abs(dist) <= r
        -r <= dist
    }
}

pub struct Fustrum {
    pub Near: Plane,
    pub Far: Plane,

    pub Left: Plane,
    pub Right: Plane,

    pub Top: Plane,
    pub Bottom: Plane,

    pub ZNear: f32,
    pub ZFar: f32,
    pub Fov: f32,
}

impl Fustrum {
    pub fn New(ZNear: f32, ZFar: f32, Fov: f32, position: na::Vector3<f32>, direction: na::Vector3<f32>, right: na::Vector3<f32>, up: na::Vector3<f32>) -> Self {
        let halfVSide = ZFar * f32::tan(Fov * 0.532);
        let halfHSide = unsafe { halfVSide * WINDOW_SIZE.0 as f32 / WINDOW_SIZE.1 as f32 };
        let frontMultFar = ZFar * direction;

        
        Self {
            Near: Plane { Normal: direction, Distance: position + ZNear * direction },
            Far: Plane { Normal: -direction, Distance: position + frontMultFar },

            Left: Plane { Normal: up.cross(&(frontMultFar + right * halfHSide)), Distance: position },
            Right: Plane { Normal: (frontMultFar - right * halfHSide).cross(&up), Distance: position },

            Top: Plane { Normal: right.cross(&(frontMultFar - up* halfVSide)), Distance: position  },
            Bottom: Plane { Normal: (frontMultFar + up * halfVSide).cross(&right), Distance: position  },

            ZFar,
            ZNear,
            Fov,
        }
    }
    
    pub fn CheckChunk(&self, chunkPos: &na::Vector3<f32>) -> bool{
        self.Near.ChunkCollidesWith(chunkPos) && self.Far.ChunkCollidesWith(chunkPos)
        && self.Left.ChunkCollidesWith(chunkPos) && self.Right.ChunkCollidesWith(chunkPos)
        && self.Bottom.ChunkCollidesWith(chunkPos) && self.Top.ChunkCollidesWith(chunkPos)
    }
}