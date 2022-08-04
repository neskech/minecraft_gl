use glium::glutin::event::{ModifiersState, MouseButton, VirtualKeyCode};

//TODO put all of this in mod.rs


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

//TODO test this in action. Make it so that we do enum::variant (ref data) <- the 'ref data' piece in the macro
#[macro_export]
macro_rules! DispatchParams { 
    ($variant:pat, $dispatcher:expr, $function:expr, $self:expr) => {
        if let $variant = & $dispatcher.Event {
            $dispatcher.DispatchWithParams($function, $self, params);
        }
    }; 
}

#[macro_export]
macro_rules! DispatchMut { 
    ($variant:pat, $dispatcher:expr, $function:expr, $self:expr) => {
        if let $variant = $dispatcher.Event {
            $dispatcher.DispatchMut($function, $self);
        }
    }; 
}

#[macro_export]
macro_rules! DispatchMutParams { 
    ($variant:pat, $dispatcher:expr, $function:expr, $self:expr) => {
        if let $variant = $dispatcher.Event {
            $dispatcher.DispatchMut($function, $self,params);
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

    pub fn DispatchWithParams<T, F, P>(&mut self, func: F, s: &T, params: &P)
    where F: Fn(&T, &P) -> bool
    {
       if self.Handled { return; }
       self.Handled = func(s, params);
    }

    pub fn DispatchMut<T, F>(&mut self, func: F, s: &mut T)
    where F: Fn(&mut T, &Event) -> bool
    {
       if self.Handled { return; }
       self.Handled = func(s, &self.Event);
    }

    pub fn DispatchMutWithParams<T, F, P>(&mut self, func: F, s: &mut T, params: &P)
    where F: Fn(&mut T, &P) -> bool
    {
       if self.Handled { return; }
       self.Handled = func(s, params);
    }
}

