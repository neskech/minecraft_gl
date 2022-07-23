
use crate::{Core::application::WINDOW_SIZE, Util::fustrum::{Fustrum, Plane}};
use nalgebra as na;

use crate::Event::event::{Event, MouseMovedEvent, KeyPressedEvent};

//intial values
const FOV: f32 = 1.2f32;
const ZNEAR: f32 = 0.01f32;
const ZFAR: f32 = 100f32;

pub struct Camera{
    pub Position: na::Vector3<f32>,

    pub CameraRight: na::Vector3<f32>,
    pub CameraUp: na::Vector3<f32>,
    pub Direction: na::Vector3<f32>,

    Pitch: f32,
    Yaw: f32,

    PreviousCursorPos: (f32, f32),

    pub Fustrum: Fustrum,
}

impl Camera{
    pub fn New() -> Self {
        Self {
            Position: na::Vector3::new(0f32, 50f32, 0f32),
            CameraRight: na::Vector3::new(0f32, 1f32, 0f32),
            CameraUp: na::Vector3::new(0f32, 0f32, 1f32),
            Pitch: 0f32,
            Yaw: 0f32,
            Direction: na::Vector3::new(0f32, 0f32, 1f32),
            PreviousCursorPos: (0f32, 0f32),
            Fustrum: Fustrum::New(ZNEAR, ZFAR, FOV, 
                na::Vector3::new(0f32, 0f32, 0f32), 
                na::Vector3::new(0f32, 0f32, 1f32), 
                na::Vector3::new(0f32, 1f32, 0f32),
                na::Vector3::new(0f32, 0f32, 1f32))
        }
    }

    pub fn GetProjectionMatrixVectorized(&self) -> na::Matrix4<f32>{
        *na::Perspective3::new(unsafe { WINDOW_SIZE.0 as f32 / WINDOW_SIZE.1 as f32 }, self.Fustrum.Fov, self.Fustrum.ZNear, self.Fustrum.ZFar).as_matrix()
    }

    pub fn GetViewMatrixVectorized(&self) -> na::Matrix4<f32>{


        // let m = na::Matrix4::new(self.CameraRight.x, self.CameraRight.y, self.CameraRight.z, 0f32,
        //                            self.CameraUp.x, self.CameraUp.y, self.CameraUp.z, 0f32,
        //                            self.Direction.x, self.Direction.y ,self.Direction.z, 0f32,
        //                            0f32, 0f32, 0f32, 0f32 );

        // let m2 = na::Matrix4::new(0f32, 0f32, 0f32, self.Position.x,
        //                                                                0f32, 0f32, 0f32, self.Position.y,
        //                                                                0f32, 0f32, 0f32, self.Position.z,        
        //                                                                0f32, 0f32, 0f32, 0f32

        //                                                           );
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

    pub fn GetViewProjection(&self) -> [[f32; 4]; 4]{
        let mat = self.GetProjectionMatrixVectorized() * self.GetViewMatrixVectorized();
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
            // let noZone = 100f32;
            // let dist = f32::sqrt((x - 400f32) * (x - 400f32) + (y - 400f32) * (y - 400f32) );
            // println!("Dist {}", dist);
            // if dist > noZone {
            //     self.Yaw -= f32::abs(x - 400f32 - 300f32) * sensitivity;
            //     self.Pitch -= f32::abs(-y - 400f32 - 300f32) * sensitivity;
            // }
            self.Pitch -= dMouse.1 * sensitivity;
            if f32::abs(self.Pitch) > 89f32 {
                self.Pitch = self.Pitch / f32::abs(self.Pitch) * 89f32;
            }
            self.Yaw += dMouse.0 * sensitivity;
            // self.Yaw= 360f32 * ( x/ 800f32 );
            // self.Pitch = 360f32 * ( y / 800f32 );

            self.Direction.x = f32::cos(f32::to_radians(self.Pitch)) * f32::cos(f32::to_radians(self.Yaw));
            self.Direction.y = f32::sin(f32::to_radians(self.Pitch));
            self.Direction.z = f32::sin(f32::to_radians(self.Yaw)) * f32::cos(f32::to_radians(self.Pitch));
            self.Direction = self.Direction.normalize();
            //println!("Yaw {} Pitch {}", self.Yaw % 360f32, self.Pitch % 360f32);
            self.CameraRight = na::Vector3::y_axis().cross(&self.Direction).normalize();
            self.CameraUp = self.Direction.cross(&self.CameraRight);
          //  println!("Direction {:?}, camRight {:?}, camUp {:?}", self.Direction, self.CameraRight, self.CameraUp);
        }
        else if let Event::KeyPressed(KeyPressedEvent { Key, ..}) = event {
            let speed = 0.3f32;
            match *Key {
                winit::event::VirtualKeyCode::S => {
                    self.Position += self.Direction * speed;
                },
                winit::event::VirtualKeyCode::W => {
                    self.Position -= self.Direction * speed;
                },
                winit::event::VirtualKeyCode::D => {
                    self.Position -= self.Direction.cross(&self.CameraUp).normalize() * speed;
                },
                winit::event::VirtualKeyCode::A => {
                    self.Position += self.Direction.cross(&self.CameraUp).normalize() * speed;
                }
                _ => {}
            }
            //println!("Position {:?}", self.Position);
        }
        self.UpdateFustrum();
    }

    fn UpdateFustrum(&mut self){
        let halfVSide = self.Fustrum.ZFar * f32::tan(self.Fustrum.Fov * 0.5f32);
        let halfHSide = unsafe { halfVSide * (WINDOW_SIZE.0 as f32 / WINDOW_SIZE.1 as f32) };
        let frontMultFar = self.Fustrum.ZFar * self.Direction;

        self.Fustrum.Near = Plane::New(self.Direction, self.Position + self.Fustrum.ZNear * self.Direction);
        self.Fustrum.Far = Plane::New( -self.Direction, self.Position + frontMultFar);

        self.Fustrum.Left = Plane::New(self.CameraUp.cross(&(frontMultFar + self.CameraRight * halfHSide)),self.Position);
        self.Fustrum.Right = Plane::New((frontMultFar - self.CameraRight * halfHSide).cross(&self.CameraUp), self.Position);

        self.Fustrum.Top = Plane::New(self.CameraRight.cross(&(frontMultFar - self.CameraUp * halfVSide)), self.Position);
        self.Fustrum.Bottom = Plane::New((frontMultFar + self.CameraUp * halfVSide).cross(&self.CameraRight), self.Position);

  
    }
}