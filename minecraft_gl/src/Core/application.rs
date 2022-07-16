use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use crate::Dispatch;
use crate::Event::event::Event;
use crate::Event::event::EventDispatcher;
use crate::Event::event::KeyPressedEvent;
use crate::Event::event::KeyReleasedEvent;
use crate::Event::event::MouseButtonPressedEvent;
use crate::Event::event::MouseButtonReleasedEvent;
use crate::Event::event::MouseMovedEvent;
use crate::Event::event::MouseScrollEvent;
use crate::Event::event::WindowCloseEvent;
use crate::Event::event::WindowResizeEvent;
use crate::Scene::sceneManager::SceneManager;
use glium::Surface;
use glium::glutin::Api;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::ElementState;
use glium::glutin::event::KeyboardInput;
use glium::glutin::event::MouseScrollDelta;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::platform::macos::WindowBuilderExtMacOS;
use glium::glutin::window::Icon;
use glium::{glutin};
use queues::*;


pub struct Application{
    EventBus: Rc<RefCell<Queue<Event>>>,
    DisplaySize: (u32, u32)

}

impl Application{
    pub fn New(size: (u32, u32)) -> Self {
        Self {
            EventBus: Rc::new(RefCell::new(Queue::new())),
            DisplaySize: size
        }
    }

    fn InitApp(&self) -> (EventLoop<()>, glium::Display){
        let eventLoop = glutin::event_loop::EventLoop::new();

        let imageIcon = image::load(Cursor::new(&include_bytes!("../../assets/misc/joe.jpeg")),
        image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let dimensions = imageIcon.dimensions();
        let icon = Icon::from_rgba(imageIcon.into_vec(), dimensions.0, dimensions.1).unwrap();

        let wb = glutin::window::WindowBuilder::new()
        .with_title("Minecraft!")
        .with_inner_size(PhysicalSize {
            width: self.DisplaySize.0,
            height: self.DisplaySize.1,
        })
        .with_window_icon(Some(icon))
        .with_resizable(true)
        .with_movable_by_window_background(true);

        let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_gl(glutin::GlRequest::Specific(Api::OpenGl, (4, 1)))
        .with_vsync(true);

        let display = glium::Display::new(wb, cb, &eventLoop).unwrap();
        (eventLoop, display)
    }

    #[allow(unused_assignments)]
    pub fn Run(mut self){

        let mut then: std::time::SystemTime = std::time::SystemTime::now();
        let mut now: std::time::SystemTime = std::time::SystemTime::now();
        let mut delta = 0f32;

        let (eventLoop, display) = self.InitApp();
        let mut sceneManager = SceneManager::New(&display);

        eventLoop.run(move |event, _, control_flow| {

    
            match event {
                glutin::event::Event::RedrawRequested(_) => {

                    self.HandleEvents(&mut sceneManager, control_flow);
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    sceneManager.Render(&mut target);
                    target.finish().unwrap();

                    now = std::time::SystemTime::now();
                    delta = now.duration_since(then).unwrap().as_secs_f32();
                   // println!("Delta {}", 1f32 / delta);
                    then = std::time::SystemTime::now();

                },
                glutin::event::Event::WindowEvent { event, .. } =>  {
                    self.ReadEvent(event);
                    return;
                },
                glutin::event::Event::MainEventsCleared => {
                    // all events have been handled
                    display.gl_window().window().request_redraw();
                },
                _ => (),
            }

        });
    }

    pub fn ReadEvent(&mut self, event: glutin::event::WindowEvent){
        use glutin::event::WindowEvent;
        match event {
            WindowEvent::CursorMoved { position: PhysicalPosition {x, y}, .. } => {
                self.EventBus.borrow_mut().add(Event::MouseMoved(MouseMovedEvent{ X: x as f32, Y: y as f32 })).unwrap();
            },
            WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, modifiers,..}, ..} => {
                if let ElementState::Pressed = state {
                    self.EventBus.borrow_mut().add(Event::KeyPressed(KeyPressedEvent{ Key: key, Mods: modifiers })).unwrap();
                }
                else {
                    self.EventBus.borrow_mut().add(Event::KeyReleased(KeyReleasedEvent{ Key: key })).unwrap();
                }
            },
            WindowEvent::MouseInput { state, button, modifiers, .. } => {
                if let ElementState::Pressed = state {
                    self.EventBus.borrow_mut().add(Event::MousePressed(MouseButtonPressedEvent { MouseButton: button, Mods: modifiers })).unwrap();
                }
                else {
                    self.EventBus.borrow_mut().add(Event::MouseReleased(MouseButtonReleasedEvent { MouseButton: button })).unwrap();
                }
            },
            WindowEvent::MouseWheel { delta , .. } => {
                let d = if let MouseScrollDelta::LineDelta(x, y) = delta{
                    (x, y)
                }
                else if let MouseScrollDelta::PixelDelta(PhysicalPosition {x, y}) = delta{
                    (x as f32, y as f32)
                }
                else {return};
                self.EventBus.borrow_mut().add(Event::MouseScrolled(MouseScrollEvent { ScrollX: d.0, ScrollY: d.1})).unwrap();
            },
            WindowEvent::Resized(size) => {
                self.EventBus.borrow_mut().add(Event::WindowResize(WindowResizeEvent { Width: size.width, Height: size.height })).unwrap();
            },
            WindowEvent::CloseRequested => {
                self.EventBus.borrow_mut().add(Event::WindowClose(WindowCloseEvent{})).unwrap();
            }
            _ => return
        }
    }

    pub fn HandleEvents(&mut self, sceneManager: &mut SceneManager, controlFlow: &mut glutin::event_loop::ControlFlow){
        while self.EventBus.borrow().size() > 0 {

            let ev: Event = self.EventBus.borrow_mut().remove().unwrap();
            let mut dispatcher = EventDispatcher::New(&ev);
            Dispatch!(Event::WindowClose(..), dispatcher, Self::OnWindowClose, self);
            if dispatcher.Handled {
                *controlFlow = glutin::event_loop::ControlFlow::Exit;
                return;
            }
            Dispatch!(Event::WindowResize(..), dispatcher, Self::OnWindowResize, self);


            //Call OnEvent w/ scene
            if !dispatcher.Handled {
               sceneManager.OnEvent(&ev);
            }
        }
    }

    pub fn OnWindowResize(&self, event: &Event) -> bool{
        if let Event::WindowResize(e) = event{
           // self.Window.borrow_mut().Resize((e.Width, e.Height));
            return true;
        }
        false
    }

    pub fn OnWindowClose(&self, _: &Event) -> bool{
        //self.Window.borrow_mut().GlfwWindow.set_should_close(true);
        true
    }

}
