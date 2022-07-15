
use nalgebra as na;

use crate::Event::event::{Event, MouseMovedEvent};

const FOV: f32 = 90f32;
const ZNEAR: f32 = 0.01f32;
const ZFAR: f32 = 100f32;

pub struct Camera{
    Position: na::Vector3<f32>,
    Pitch: f32,
    Yaw: f32,
    //TODO change the mouse moved event's type to F32
    Direction: na::Vector3<f32>,
    PreviousCursorPos: (f32, f32),
}

impl Camera{
    pub fn New() -> Self {
        Self {
            Position: na::Vector3::zeros(),
            Pitch: 0f32,
            Yaw: 0f32,
            Direction: na::Vector3::zeros(),
            PreviousCursorPos: (0f32, 0f32),
        }
    }

    pub fn GetProjectionMatrix(&self) -> na::Matrix4<f32>{
        *na::Perspective3::new(1f32, FOV, ZNEAR, ZFAR).as_matrix()
    }

    pub fn GetViewMatrix(&self) -> na::Matrix4<f32>{
        let up: na::Vector3<f32> = na::Vector3::new(0.0f32, 1.0f32, 0.0f32); 
        let camRight = up.cross(&self.Direction).normalize();
        let camUp = self.Direction.cross(&camRight);


        let m = na::Matrix4::new(camRight.x, camRight.y, camRight.z, 0f32,
                                   camUp.x, camUp.y, camUp.z, 0f32,
                                   self.Direction.x, self.Direction.y ,self.Direction.z, 0f32,
                                   0f32, 0f32, 0f32, 0f32 );

        let m2 = na::Matrix4::new(0f32, 0f32, 0f32, self.Position.x,
                                                                       0f32, 0f32, 0f32, self.Position.y,
                                                                       0f32, 0f32, 0f32, self.Position.z,        
                                                                       0f32, 0f32, 0f32, 0f32

                                                                  );

        m * m2
    }

    pub fn OnEvent(&mut self, event: &Event){
        if let Event::MouseMoved(MouseMovedEvent{X: x, Y: y}) = event {
            let dMouse = (*x as f32 - self.PreviousCursorPos.0, *y as f32 - self.PreviousCursorPos.1);
            self.PreviousCursorPos = (*x as f32, *y as f32);
            
            self.Pitch += dMouse.0;
            self.Yaw += dMouse.1;

            self.Direction.x = f32::cos(f32::to_radians(self.Pitch)) * f32::cos(f32::to_radians(self.Yaw));
            self.Direction.y = f32::sin(f32::to_radians(self.Pitch));
            self.Direction.z = f32::sin(f32::to_radians(self.Yaw)) * f32::cos(f32::to_radians(self.Pitch));

        }
    }
}