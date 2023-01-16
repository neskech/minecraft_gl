
use glium::glutin::event::VirtualKeyCode;
use crate::{Core::application::WINDOW_SIZE, Util::fustrum::Fustrum};
use nalgebra as na;
use crate::Event::event::{Event, MouseMovedEvent, KeyPressedEvent};

//intial values
const FOV: f32 = 1.2f32;
const ZNEAR: f32 = 0.01f32;
const ZFAR: f32 = 200f32;

pub struct Camera{
    pub Position: na::Vector3<f32>,

    pub CameraRight: na::Vector3<f32>,
    pub CameraUp: na::Vector3<f32>,
    pub Direction: na::Vector3<f32>,

    Pitch: f32,
    Yaw: f32,

    PreviousCursorPos: (f32, f32),

    pub Fustrum: Fustrum,
    Fov: f32,
    ZNear: f32,
    ZFar: f32,
}

impl Camera{
    pub fn New() -> Self {
        let mut self_ = Self {
            Position: na::Vector3::new(0f32, 50f32, 0f32),
            CameraRight: na::Vector3::new(0f32, 1f32, 0f32),
            CameraUp: na::Vector3::new(0f32, 0f32, 1f32),
            Pitch: 0f32,
            Yaw: 0f32,
            Direction: na::Vector3::new(0f32, 0f32, 1f32),
            PreviousCursorPos: (0f32, 0f32),
            Fustrum: Fustrum::New(&na::Matrix4::zeros()),

            Fov: FOV,
            ZNear: ZNEAR,
            ZFar: ZFAR
        };
        self_.Fustrum.update(&self_.GetViewProjection());
        self_
    }

    pub fn GetViewProjection(&self) -> na::Matrix4<f32>{
        self.GetProjectionMatrixVectorized() * self.GetViewMatrixVectorized() 
    }

    pub fn GetProjectionMatrixVectorized(&self) -> na::Matrix4<f32>{
        *na::Perspective3::new(unsafe { WINDOW_SIZE.0 as f32 / WINDOW_SIZE.1 as f32 }, self.Fov, self.ZNear, self.ZFar).as_matrix()
    }

    pub fn GetViewMatrixVectorized(&self) -> na::Matrix4<f32>{
        let eye = na::Point3::new(self.Position.x, self.Position.y, self.Position.z);
        let target = na::Point3::new(self.Position.x + self.Direction.x, self.Position.y + self.Direction.y, self.Position.z + self.Direction.z);
        na::Isometry3::look_at_lh(&eye, &target, &self.CameraUp).to_matrix()
    }

    pub fn GetViewMatrix(&self) -> [[f32; 4]; 4]{
        let mat = self.GetViewMatrixVectorized();
       // println!("MATRIX {:?}", mat);
        [
            [mat[0], mat[1], mat[2], mat[3]],
            [mat[4], mat[5], mat[6], mat[7]],
            [mat[8], mat[9], mat[10], mat[11]],
            [mat[12], mat[13], mat[14], mat[15]],

        ]
    }

    pub fn GetProjectionMatrix(&self) -> [[f32; 4]; 4]{
        let mat = self.GetProjectionMatrixVectorized();
        [
            [mat[0], mat[1], mat[2], mat[3]],
            [mat[4], mat[5], mat[6], mat[7]],
            [mat[8], mat[9], mat[10], mat[11]],
            [mat[12], mat[13], mat[14], mat[15]],

        ]
    }

    pub fn OnEvent(&mut self, event: &Event){
        if let Event::MouseMoved(MouseMovedEvent{X: x, Y: y}) = event {
            let dMouse = (*x as f32 - self.PreviousCursorPos.0, *y as f32 - self.PreviousCursorPos.1);
            self.PreviousCursorPos = (*x as f32, *y as f32);
            
            let sensitivity = 0.5f32;
            self.Pitch -= dMouse.1 * sensitivity;

            if f32::abs(self.Pitch) > 89f32 {
                self.Pitch = self.Pitch / f32::abs(self.Pitch) * 89f32;
            }
            self.Yaw += dMouse.0 * sensitivity;

            self.Direction.x = f32::cos(f32::to_radians(self.Pitch)) * f32::cos(f32::to_radians(self.Yaw));
            self.Direction.y = f32::sin(f32::to_radians(self.Pitch));
            self.Direction.z = f32::sin(f32::to_radians(self.Yaw)) * f32::cos(f32::to_radians(self.Pitch));
            self.Direction = self.Direction.normalize();

            self.CameraRight = na::Vector3::y_axis().cross(&self.Direction).normalize();
            self.CameraUp = self.Direction.cross(&self.CameraRight);

        }
        else if let Event::KeyPressed(KeyPressedEvent { Key, ..}) = event {
            let speed = 1.9f32;
            match *Key {
                VirtualKeyCode::S => {
                    self.Position += self.Direction * speed;
                },
                VirtualKeyCode::W => {
                    self.Position -= self.Direction * speed;
                },
                VirtualKeyCode::D => {
                    self.Position -= self.Direction.cross(&self.CameraUp).normalize() * speed;
                },
                VirtualKeyCode::A => {
                    self.Position += self.Direction.cross(&self.CameraUp).normalize() * speed;
                }
                _ => {}
            }
        }
        self.UpdateFustrum();
    }

    fn UpdateFustrum(&mut self){
        self.Fustrum.update(&self.GetViewProjection());
  
    }
}