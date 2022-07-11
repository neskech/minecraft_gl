use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::Core::window::Window;
use crate::Core::input::KeyListener;
use crate::Core::input::MouseListener;
use crate::Dispatch;
use crate::Event::event::Event;
use crate::Event::event::EventDispatcher;
use crate::Event::eventBus::*;

pub struct Application{
    Window: RefCell<Window>,
    KeyListener: KeyListener,
    MouseListener: MouseListener,
    EventBus: Rc<RefCell<EventBus>>,

}

impl Application{
    pub fn New(defaultSize: (i32, i32)) -> Self {
         let bus = Rc::new(RefCell::new(EventBus::New()));
         Self {
            Window: RefCell::new(Window::New(defaultSize, &bus)),
            KeyListener: KeyListener::New(),
            MouseListener: MouseListener::New(),
            EventBus: bus
        }

    }

    pub fn Init(& mut self){
        
    }

    pub fn Run(&self){

        let mut then = 0f32;
        let mut now: f32;
        let mut delta: f32;

        while !self.Window.borrow().ShouldClose() {
            now = self.Window.borrow().GetTime() as f32;
            delta = now - then;
            then = self.Window.borrow().GetTime() as f32;

            println!("Delta: {}", 1f32/delta);

            self.HandleEvents();

            //call on scene

            self.Window.borrow_mut().SwapBuffers();
            self.Window.borrow_mut().ProcessEvents();
        }

    }

    pub fn HandleEvents(&self){
        while self.EventBus.deref().borrow().Size() > 0 {

            let ev: Event = self.EventBus.deref().borrow_mut().Remove();
            let mut dispatcher = EventDispatcher::New(&ev);
            Dispatch!(Event::WindowResize(..), dispatcher, Self::OnWindowResize, self);
            Dispatch!(Event::WindowClose(..), dispatcher, Self::OnWindowClose, self);

            //Call OnEvent w/ scene
            if !dispatcher.Handled {

            }
        }
    }

    pub fn OnWindowResize(&self, event: &Event) -> bool{
        if let Event::WindowResize(e) = event{
            self.Window.borrow_mut().Resize((e.Width, e.Height));
            return true;
        }
        false
    }

    pub fn OnWindowClose(&self, _: &Event) -> bool{
        self.Window.borrow_mut().GlfwWindow.set_should_close(true);
        true
    }

}
