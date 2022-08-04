use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use crate::Dispatch;
use crate::Event::event::*;
use crate::Scene::sceneManager::SceneManager;
use glium::Surface;
use glium::glutin::Api;
use glium::glutin::dpi::PhysicalPosition;
use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode};
use glium::glutin::event_loop::EventLoop;
//use glium::glutin::platform::macos::WindowBuilderExtMacOS;
use glium::glutin::window::Icon;
use glium::glutin;
use queues::*;

//Many parts of the application will need to access the window size. As such, its a global variable
//This should only ever be modified inside of this file
pub static mut WINDOW_SIZE: (u32, u32) = (800, 800);

pub struct Application{
    EventBus: Rc<RefCell<Queue<Event>>>,

}

impl Application{
    pub fn New() -> Self {
        Self {
            EventBus: Rc::new(RefCell::new(Queue::new())),
        }
    }

    fn InitApp(&self) -> (EventLoop<()>, glium::Display){
        //create the event loop
        let eventLoop = glutin::event_loop::EventLoop::new();

        //Load in an icon for the window
        let imageIcon = image::load(Cursor::new(&include_bytes!("../../assets/misc/joe.jpeg")),
        image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let dimensions = imageIcon.dimensions();
        let icon = Icon::from_rgba(imageIcon.into_vec(), dimensions.0, dimensions.1).unwrap();

        //construct the window
        let wb = glutin::window::WindowBuilder::new()
        .with_title("Minecraft!")
        .with_inner_size(PhysicalSize {
            width: unsafe { WINDOW_SIZE.0 },
            height: unsafe { WINDOW_SIZE.1 },
        })
        .with_window_icon(Some(icon))
        .with_resizable(true);
        //.with_movable_by_window_background(true);

        //Construct the context
        let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_gl(glutin::GlRequest::Specific(Api::OpenGl, (4, 1)))
        .with_vsync(true);

        //construct the display and return it along with the event loop
        let display = glium::Display::new(wb, cb, &eventLoop).unwrap();
        display.gl_window().window().set_cursor_visible(false);

        (eventLoop, display)
    }

    #[allow(unused_assignments)]
    pub fn Run(mut self){
        //Will get a compile error if these variables aren't initialized since the closure moves them
        let mut then: std::time::SystemTime = std::time::SystemTime::now();
        let mut now: std::time::SystemTime = std::time::SystemTime::now();
        let mut delta = 0f32;

        let (eventLoop, display) = self.InitApp();
        let mut sceneManager = SceneManager::New(&display);

        eventLoop.run(move |event, _, control_flow| {

            match event {
                glutin::event::Event::RedrawRequested(_) => {
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                    //get the delta time
                    now = std::time::SystemTime::now();
                    delta = now.duration_since(then).unwrap().as_secs_f32();
                    //println!("{}", 1f32 / delta);
                   // println!("Delta {}", 1f32 / delta);
                    then = std::time::SystemTime::now();
                    
                    //main game logic happens here
                    self.HandleEvents(&mut sceneManager, control_flow);
                    sceneManager.Render(&mut target);
                    sceneManager.Update(delta);

                    target.finish().unwrap();

                },
                glutin::event::Event::WindowEvent { event, .. } =>  {
                    self.ReadEvent(event);
                    return;
                },
                glutin::event::Event::MainEventsCleared => {
                    display.gl_window().window().request_redraw();
                },
                _ => (),
            }

        });
    }

    #[allow(deprecated)]
    pub fn ReadEvent(&mut self, event: glutin::event::WindowEvent){
        use glutin::event::WindowEvent;
        //We have our own event enum, so take glutin's window events and convert them into ours
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

    pub fn HandleEvents(&self, sceneManager: &mut SceneManager, controlFlow: &mut glutin::event_loop::ControlFlow){
        //Handle all events within the event bus. Events triggered all over the application will be handled and propogated down here
        while self.EventBus.borrow().size() > 0 {
            let ev: Event = self.EventBus.borrow_mut().remove().unwrap();
            let mut dispatcher = EventDispatcher::New(&ev);

            //dispatch the window close event
            Dispatch!(Event::WindowClose(..), dispatcher, Self::OnWindowClose, self);
            
            //If the event was handled (aka the event was a window close event) or the excape key was pressed, exit the game
            if dispatcher.Handled {
                *controlFlow = glutin::event_loop::ControlFlow::Exit;
                return;
            }
            else if let Event::KeyPressed(KeyPressedEvent {Key: VirtualKeyCode::Escape, ..}) = ev {
                *controlFlow = glutin::event_loop::ControlFlow::Exit;
                return;
            }

            //Handle window resize
            Dispatch!(Event::WindowResize(..), dispatcher, Self::OnWindowResize, self);

            //Propogate down the event to the scenemanager
            if !dispatcher.Handled {
               sceneManager.OnEvent(&ev);
            }
        }
    }

    pub fn OnWindowResize(&self, event: &Event) -> bool{
        if let Event::WindowResize(WindowResizeEvent { Width, Height }) = event {
            unsafe { WINDOW_SIZE = (*Width, *Height) };
        }
        true
    }

    pub fn OnWindowClose(&self, _: &Event) -> bool{
        true
    }

}