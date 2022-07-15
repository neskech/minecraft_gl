extern crate glfw;
use core::ffi::CStr;
use std::ffi::CString;
use std::{sync::mpsc::Receiver, ops::Deref};
use glfw::{Key, Action, Context};
use crate::Event::event::*;
use crate::Event::eventBus::*;
use std::rc::Rc;
use std::cell::RefCell;


pub struct Window{
    pub GlfwWindow: glfw::Window,
    pub GLFW: glfw::Glfw,
    pub Events: Receiver<(f64, glfw::WindowEvent)>,
    pub Size: (i32, i32),
    pub Minimized: bool,
    EventBus: Rc<RefCell<EventBus>>,

}

impl Window{
    pub fn New(size: (i32, i32), eventBus: &Rc<RefCell<EventBus>>) -> Self{
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw::Glfw::window_hint(&mut glfw, glfw::WindowHint::ContextVersion(4, 1));
       glfw::Glfw::window_hint(&mut glfw, glfw::WindowHint::OpenGlForwardCompat(true));
       glfw::Glfw::window_hint(&mut glfw, glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        //TODO turn the window size back to a u32
        let (mut window, events) = glfw.create_window(size.0 as u32, size.1 as u32, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        Self::SetPollingTypes(&mut window);

        window.make_current();
        
        gl::load_with(|s|  window.get_proc_address(s) as *const _ );



    // glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    // glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);
    // glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
    // glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

        return Self{
            GlfwWindow: window,
            GLFW: glfw,
            Events: events,
            Size: size,
            Minimized: false,
            EventBus: Rc::clone(eventBus),
        }
    }

    fn SetPollingTypes(window: &mut glfw::Window){
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_close_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_size_polling(true);
        window.set_maximize_polling(true);
    }

    pub fn ProcessEvents(&mut self){
        self.GLFW.poll_events();
        for (_, event) in glfw::flush_messages(&self.Events) {
            self.HandleWindowEvent(event);
        }
    }


    fn HandleWindowEvent(&self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _ ) => {
                self.EventBus.deref().borrow_mut().AddEvent(Event::WindowClose(WindowCloseEvent{}));
            },
            glfw::WindowEvent::Key(key, _, action, mods) => {
                self.EventBus.deref().borrow_mut().AddEvent(Event::KeyPressed(KeyPressedEvent{Key: key, Action: action, Mods: mods}));
            },
            glfw::WindowEvent::MouseButton(mouseButton, _,  mods) => {
                self.EventBus.deref().borrow_mut().AddEvent(Event::MousePressed(MouseButtonPressedEvent{MouseButton: mouseButton, Mods: mods}));
            },
            glfw::WindowEvent::CursorPos(x, y) => {
                self.EventBus.deref().borrow_mut().AddEvent(Event::MouseMoved(MouseMovedEvent{X: x, Y: y}));
            },
            glfw::WindowEvent::Size(width, height) => {
                self.EventBus.deref().borrow_mut().AddEvent(Event::WindowResize(WindowResizeEvent{Width: width, Height: height}));
            },
            glfw::WindowEvent::Scroll(x, y) => {
                self.EventBus.deref().borrow_mut().AddEvent(Event::MouseScrolled(MouseScrollEvent{ScrollX: x, ScrollY: y}));
            },
            glfw::WindowEvent::Maximize(_) => {
                self.EventBus.deref().borrow_mut().AddEvent(Event::WindowMaximize(WindowMaximizeEvent{}));
            }
            _ => {}
        }

       
    }

    pub fn SwapBuffers(&mut self){
        self.GlfwWindow.swap_buffers();
    }

    pub fn ClearBuffers(&mut self){
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(1f32, 0.2f32, 0.1f32, 1f32);
        }
    }

    pub fn GetTime(&self) -> f64{
        self.GLFW.get_time()
    }

    pub fn Resize(&mut self, newSize: (i32, i32)){
        self.Size = newSize;
        self.GlfwWindow.set_size(self.Size.0, self.Size.1);
    }

    pub fn ShouldClose(&self) -> bool{
        self.GlfwWindow.should_close()
    }

}
