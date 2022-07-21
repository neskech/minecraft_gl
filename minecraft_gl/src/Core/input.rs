// use glfw::{Key, Action, Modifiers, MouseButton};


// pub struct KeyListener{
//     KeyCodes: [u8; 350],
// }

// //TODO utilize tuple structs for (X, Y), (Dx, Dy), (ScrollX, ScrollY), (DScrollX, DScrollY)
// pub struct MouseListener{
//     MousePos: (f64, f64),
//     MousePosChange: (f64, f64),

//     Scroll: (f64, f64),
//     ScrollChange: (f64, f64),

//     MouseCodes: [u8; 8],
// }

// impl KeyListener{
//     pub fn New() -> Self{
//         Self { KeyCodes: [0; 350] }
//     }

//     pub fn KeyPressedCallback(&mut self, keyCode: Key, mods: Modifiers, action: Action){
//         let keyRef: &mut u8 = &mut self.KeyCodes[keyCode as usize]; //to avoid writing the array indexing each time

//         //first bit is the state (pressed or released)
//         //second bit is for key repeat
//         match action {
//             Action::Release => { *keyRef = 0; return; }, //0 = 0b00000000
//             Action::Press =>  *keyRef = 1, //1 = 0b10000000
//             Action::Repeat => *keyRef |= 3, //3 = 0b11 -> Second bit for repeat
//         }
        
//         //add the modifier bits (6 bits)
//         //Make the first two bits 0
//         *keyRef |= mods.bits() as u8 >> 2;
        
//     }

//     pub fn IsKeyPressed(&self, keyCode: Key, mods: Option<&[Modifiers]>) -> bool{
//         let keyRef: &u8 = &self.KeyCodes[keyCode as usize]; //to avoid writing the array indexing each time
//         let mut sat: bool = *keyRef & 1 == 1;

//         if let Some(modifiers) = mods {
//             for modifier in modifiers {
//                 //check last 6 bits against the modifier
//                 //The sat &&... is for if the *keyRef & 1 == 1 was false
//                 sat = sat && *keyRef << 2 == modifier.bits() as u8;
//                 if !sat {
//                     return false;
//                 }
//             }
//         }

//         sat
//     }

//     pub fn IsKeyRepeated(self, keyCode: Key, mods: Option<&[Modifiers]>) -> bool{
//         let keyRef: &u8 = &self.KeyCodes[keyCode as usize]; //to avoid writing the array indexing each time
//         let mut sat: bool = *keyRef & 3 == 3; //first two bits must be 1

//         if let Some(modifiers) = mods {
//             for modifier in modifiers {
//                 //check last 6 bits against the modifier
//                 //The sat &&... is for if the *keyRef & 1 == 1 was false
//                 sat = sat && *keyRef << 2 == modifier.bits() as u8;
//                 if !sat {
//                     return false;
//                 }
//             }
//         }

//         sat
//     }

// }

// impl MouseListener{
//     pub fn New() -> Self{
//         Self { 
//             MousePos: (0f64, 0f64),
//             MousePosChange: (0f64, 0f64),
//             Scroll: (0f64, 0f64),
//             ScrollChange: (0f64, 0f64),
//             MouseCodes: [0; 8] 
//         }
//     }

//     pub fn MouseCharacterCallback(&mut self, mouseButton: MouseButton, mods: Modifiers, action: Action){
//         let mouseRef: &mut u8 = &mut self.MouseCodes[mouseButton as usize]; //to avoid writing the array indexing each time

//         //first bit is the state (pressed or released)
//         //second bit is for key repeat
//         match action {
//             Action::Release => { *mouseRef = 0; return; }, //0 = 0b00000000
//             Action::Press =>  *mouseRef = 1, //1 = 0b10000000
//             Action::Repeat => *mouseRef |= 3, //3 = 0b11 -> Second bit for repeat
//         }
        
//         //add the modifier bits (6 bits)
//         //Make the first two bits 0
//         *mouseRef |= mods.bits() as u8 >> 2;
        
//     }

//     pub fn MouseScrollCallback(&mut self, scroll: (f64, f64)){
//         self.ScrollChange = (scroll.0 - self.Scroll.0, scroll.1 - self.Scroll.1);
//         self.Scroll = scroll;
//     }

//     pub fn MousePressedCallback(&mut self, mousePos: (f64, f64)){
//         self.MousePosChange = (mousePos.0 - self.MousePos.0, mousePos.1 - self.MousePos.1);
//         self.MousePos = mousePos;
//     }

//     pub fn IsMouseButtonPressed(&self, mouseButton: MouseButton, mods: Option<&[Modifiers]>) -> bool{
//         let mouseRef: &u8 = &self.MouseCodes[mouseButton as usize]; //to avoid writing the array indexing each time
//         let mut sat: bool = *mouseRef & 1 == 1;

//         if let Some(modifiers) = mods {
//             for modifier in modifiers {
//                 //check last 6 bits against the modifier
//                 //The sat &&... is for if the *keyRef & 1 == 1 was false
//                 sat = sat && *mouseRef << 2 == modifier.bits() as u8;
//                 if !sat {
//                     return false;
//                 }
//             }
//         }

//         sat
//     }

//     pub fn IsMouseButtonRepeated(&self, mouseButton: MouseButton, mods: Option<&[Modifiers]>) -> bool{
//         let mouseRef: &u8 = &self.MouseCodes[mouseButton as usize]; //to avoid writing the array indexing each time
//         let mut sat: bool = *mouseRef & 3 == 3; //first two bits must be 1

//         if let Some(modifiers) = mods {
//             for modifier in modifiers {
//                 //check last 6 bits against the modifier
//                 //The sat &&... is for if the *keyRef & 1 == 1 was false
//                 sat = sat && *mouseRef << 2 == modifier.bits() as u8;
//                 if !sat {
//                     return false;
//                 }
//             }
//         }

//         sat
//     }

//     //Getters...

//     pub fn GetMouseCoords(&self) -> (f64, f64){
//         self.MousePos
//     }
//     pub fn GetMouseX(&self) -> f64{
//         self.MousePos.0
//     }
//     pub fn GetMouseY(&self) -> f64{
//         self.MousePos.1
//     }

//     pub fn GetMouseDxDy(&self) -> (f64, f64){
//         self.MousePosChange
//     }
//     pub fn GetMouseDx(&self) -> f64{
//         self.MousePosChange.0
//     }
//     pub fn GetMouseDy(&self) -> f64{
//         self.MousePosChange.1
//     }

//     pub fn GetMouseScroll(&self) -> (f64, f64){
//         self.Scroll
//     }
//     pub fn GetScrollX(&self) -> f64{
//         self.Scroll.0
//     }
//     pub fn GetScrollY(&self) -> f64{
//         self.Scroll.1
//     }
//     pub fn GetScrollDxDy(&self) -> (f64, f64){
//         self.ScrollChange
//     }
//     pub fn GetScrollDx(&self) -> f64{
//         self.ScrollChange.0
//     }
//     pub fn GetScrollDy(&self) -> f64{
//         self.ScrollChange.1
//     }


    
// }