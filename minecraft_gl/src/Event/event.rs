use glium::glutin::event::{ModifiersState, MouseButton, VirtualKeyCode};




#[derive(Clone)]
pub enum Event{
    MouseMoved(MouseMovedEvent),
    MousePressed(MouseButtonPressedEvent),
    MouseReleased(MouseButtonReleasedEvent),
    MouseScrolled(MouseScrollEvent),
    KeyPressed(KeyPressedEvent),
    KeyReleased(KeyReleasedEvent),
    WindowResize(WindowResizeEvent),
    WindowMaximize(WindowMaximizeEvent),
    WindowClose(WindowCloseEvent),
}


#[derive(Clone)]
pub struct MouseMovedEvent{
    pub X: f32,
    pub Y: f32,
}

#[derive(Clone)]
pub struct MouseButtonPressedEvent{
    pub MouseButton: MouseButton,
    pub Mods: ModifiersState,
}

#[derive(Clone)]
pub struct MouseButtonReleasedEvent{
    pub MouseButton: MouseButton,
}

#[derive(Clone)]
pub struct MouseScrollEvent{
    pub ScrollX: f32,
    pub ScrollY: f32
}

#[derive(Clone)]
pub struct KeyPressedEvent{
    pub Key: VirtualKeyCode,
    pub Mods: ModifiersState,
}

#[derive(Clone)]
pub struct KeyReleasedEvent{
    pub Key: VirtualKeyCode,
}

#[derive(Clone)]
pub struct WindowResizeEvent{
    pub Width: u32,
    pub Height: u32,
}

#[derive(Clone)]
pub struct WindowCloseEvent{

}
#[derive(Clone)]
pub struct WindowMaximizeEvent{
    
}

#[macro_export]
macro_rules! Dispatch { 
    ($variant:pat, $dispatcher:expr, $function:expr, $self:expr) => {
        if let $variant = $dispatcher.Event {
            $dispatcher.Dispatch($function, $self);
        }
    }; 
}

pub struct EventDispatcher<'a>{
    pub Event: &'a Event,
    pub Handled: bool,
}

impl<'a> EventDispatcher<'a>{
    pub fn New(event: &'a Event) -> Self{
        Self { Event: event, Handled: false }
    }
    
    //E is the event type we want to dispatch
    pub fn Dispatch<T, F>(&mut self, func: F, s: &T)
    where F: Fn(&T, &Event) -> bool
    {
       if self.Handled { return; }
       self.Handled = func(s, &self.Event);
    }
}


