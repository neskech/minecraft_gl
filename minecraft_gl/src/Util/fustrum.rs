use crate::{Core::application::WINDOW_SIZE, World::chunk::{CHUNK_BOUNDS_X, CHUNK_BOUNDS_Y, CHUNK_BOUNDS_Z}};
use nalgebra as na;

pub struct Plane {
    pub Normal: na::Vector3<f32>,
    pub Distance: f32, //distance to origin
}

impl Plane {
    pub fn New(norm: na::Vector3<f32>, point: na::Vector3<f32>) -> Self {
        let normalized = norm.normalize();
        Self {
            Normal: normalized,
            Distance: normalized.dot(&point)
        }
    }
    pub fn DistanceTo(&self, point: &na::Vector3<f32>) -> f32 {
       point.dot(&self.Normal) - self.Distance
    }

    pub fn ChunkCollidesWith(&self, chunkPos: &na::Vector3<f32>) -> bool{
        //from https://gdbooks.gitbooks.io/3dcollisions/content/Chapter2/static_aabb_plane.html
        let extents = na::Vector3::new(
             CHUNK_BOUNDS_X as f32 / 2f32,
              CHUNK_BOUNDS_Y as f32 / 2f32,
              CHUNK_BOUNDS_Z as f32 / 2f32);

        let r = extents.dot(&self.Normal.abs());
        let dist = self.DistanceTo(chunkPos);
       // f32::abs(dist) >= r
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
        let halfVSide = ZFar * f32::tan(Fov * 0.5f32);
        let halfHSide = unsafe { halfVSide * (WINDOW_SIZE.0 as f32 / WINDOW_SIZE.1 as f32) };
        let frontMultFar = ZFar * direction;

        
        Self {
            Near: Plane::New(direction, position + ZNear * direction),
            Far: Plane::New( -direction, position + frontMultFar),

            Left: Plane::New(up.cross(&(frontMultFar + right * halfHSide)),position),
            Right: Plane::New((frontMultFar - right * halfHSide).cross(&up), position),

            Top: Plane::New(right.cross(&(frontMultFar - up * halfVSide)), position),
            Bottom: Plane::New((frontMultFar + up * halfVSide).cross(&right), position),

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

pub fn FustrumCullAABB(VP: &nalgebra::Matrix4<f32>, aabb: &AABB) -> bool {
    // Use our min max to define eight corners
    let corners: [nalgebra::Vector4<f32>; 8] = [
        na::Vector4::new(aabb.Min.x, aabb.Min.y, aabb.Min.z, 1f32), // x y z
        na::Vector4::new(aabb.Max.x, aabb.Min.y, aabb.Min.z, 1f32), // X y z
        na::Vector4::new(aabb.Min.x, aabb.Max.y, aabb.Min.z, 1f32), // x Y z
        na::Vector4::new(aabb.Max.x, aabb.Max.y, aabb.Min.z, 1f32), // X Y z

        na::Vector4::new(aabb.Min.x, aabb.Min.y, aabb.Max.z, 1f32), // x y Z
        na::Vector4::new(aabb.Max.x, aabb.Min.y, aabb.Max.z, 1f32), // X y Z
        na::Vector4::new(aabb.Min.x, aabb.Max.y, aabb.Max.z, 1f32), // x Y Z
        na::Vector4::new(aabb.Max.x, aabb.Max.y, aabb.Max.z, 1f32), // X Y Z
    ];

    let mut inside = false;
    for i in 0..8 {
        // Transform vertex
        let corner = VP * corners[i];
        // Check vertex against clip space bounds
        inside = inside ||
            within(-corner.w, corner.x, corner.w) &&
            within(-corner.w, corner.y, corner.w) &&
            within(0f32, corner.z, corner.w);

    }
    inside
}

fn within(left: f32, middle: f32, right: f32) -> bool{
    middle >= left && middle <= right
}

pub struct AABB {
    pub Min: na::Vector3<f32>,
    pub Max: na::Vector3<f32>,
}