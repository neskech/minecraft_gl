use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

use crate::{Dispatch, DispatchMut};
use crate::Event::event::*;
use crate::Scene::sceneManager::SceneManager;
use queues::*;

//Many parts of the application will need to access the window size. As such, its a global variable
//This should only ever be modified inside of this file
pub static mut WINDOW_SIZE: (u32, u32) = (800, 800);

struct WpguState{
    Surface: wgpu::Surface,
    Device: wgpu::Device,
    Queue: wgpu::Queue,
    Config: wgpu::SurfaceConfiguration,
    Size: winit::dpi::PhysicalSize<u32>,
}

impl WpguState {
    async fn New(window: &winit::window::Window) -> Self {
        //Used to create Surface and Adapter
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        //Surface -> What we draw to
        let surface = unsafe { instance.create_surface(window) };
        //Adapter -> Handle tot the GPU
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(), //Low power by default
                compatible_surface: Some(&surface), //can present to the surface we made
                force_fallback_adapter: false, //if true, forces use of an adapter that works on all systems
            },
        ).await.unwrap();

        let size = window.inner_size();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                //extra features required by the application
                features: wgpu::Features::empty(),
                //certain types of resources we can create
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            //Textures will be used to write to the screen
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            //get the supported texture format. Specifies how they will be stored on the GPU
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            //Essentially vsync 
            present_mode: wgpu::PresentMode::Fifo,
        };
        //configure the surface
        surface.configure(&device, &config); 

        Self {
            Surface: surface,
            Device: device,
            Queue: queue,
            Config: config,
            Size: size,
        }
    }
}
pub struct Application{
    EventBus: Rc<RefCell<Queue<Event>>>,
    SceneManager: SceneManager,
    Wgpu: WpguState
}

impl Application{
    fn InitApp() -> (winit::window::Window, winit::event_loop::EventLoop<()>){
        //create the event loop
        let eventLoop = winit::event_loop::EventLoop::new();

        //Load in an icon for the window
        let imageIcon = image::load(Cursor::new(&include_bytes!("../../assets/misc/joe.jpeg")),
        image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let dimensions = imageIcon.dimensions();
        let icon = winit::window::Icon::from_rgba(imageIcon.into_vec(), dimensions.0, dimensions.1).unwrap();
        
        //construct the window
        let w = winit::window::WindowBuilder::new()
        .with_title("Minecraft!")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: unsafe { WINDOW_SIZE.0 },
            height: unsafe { WINDOW_SIZE.1 },
        })
        .with_window_icon(Some(icon))
        .with_resizable(true)
        .build(&eventLoop)
        .unwrap();
        //.with_movable_by_window_background(true);

        (w, eventLoop)
    }

    #[allow(unused_assignments)]
    pub async fn Run(){
        //Will get a compile error if these variables aren't initialized since the closure moves them
        let mut then: std::time::SystemTime = std::time::SystemTime::now();
        let mut now: std::time::SystemTime = std::time::SystemTime::now();
        let mut delta = 0f32;

        let (window, eventLoop) = Self::InitApp();

        let wgpu = WpguState::New(&window).await;
        let mut application = Application { 
            EventBus: Rc::new(RefCell::new(Queue::new())), 
            SceneManager: SceneManager::New(&wgpu.Device, &wgpu.Queue, &wgpu.Config), 
            Wgpu: WpguState::New(&window).await
        };

        eventLoop.run(move |event, _, control_flow| {

            match event {
                winit::event::Event::RedrawRequested(_) => {

                    //get the delta time
                    now = std::time::SystemTime::now();
                    delta = now.duration_since(then).unwrap().as_secs_f32();
                    then = std::time::SystemTime::now();
                    
                    //main game logic happens here
                    application.HandleEvents(control_flow);
                    application.SceneManager.Update(delta);
                    match application.Render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => application.Resize(application.Wgpu.Size.width, application.Wgpu.Size.height),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = winit::event_loop::ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }


                },
                winit::event::Event::WindowEvent { event, .. } =>  {
                    application.ReadEvent(event);
                    return;
                },
                winit::event::Event::MainEventsCleared => {
                    window.request_redraw();
                },
                _ => (),
            }

        });
    }

    fn Render(&mut self) -> Result<(), wgpu::SurfaceError>{
        //wait for wgpu to send us another surface texture to render to
        let output = self.Wgpu.Surface.get_current_texture()?;
        //create a texture view which allows us to modify the texture with rendering code
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        //create a command buffer, which will store our render commands before sending them to the GPU
        let mut encoder = self.Wgpu.Device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view, //render to our screen texture
                //Texture that will recieve resolved output. If multisampling isn't enabled, it's the same as the view. No need for it
                resolve_target: None,
                ops: wgpu::Operations {
                    //set the clear color
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    //Store our rendering results to the surface texture inside of the texture view
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        //Render the scene
        self.SceneManager.Render(&mut _render_pass, &self.Wgpu.Queue);
        //render pass borrows the encoder mutably. To call finish on the encoder, the mutable reference must die
        drop(_render_pass);

        //finish the command buffer and send it to the gpu
        self.Wgpu.Queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    #[allow(deprecated)]
    fn ReadEvent(&mut self, event: winit::event::WindowEvent){
        use winit::event::WindowEvent;
        
        //We have our own event enum, so take winit's window events and convert them into ours
        match event {
            WindowEvent::CursorMoved { position: winit::dpi::PhysicalPosition {x, y}, .. } => {
                self.EventBus.borrow_mut().add(Event::MouseMoved(MouseMovedEvent{ X: x as f32, Y: y as f32 })).unwrap();
            },
            WindowEvent::KeyboardInput { input: winit::event::KeyboardInput { virtual_keycode: Some(key), state, modifiers,..}, ..} => {
                if let winit::event::ElementState::Pressed = state {
                    self.EventBus.borrow_mut().add(Event::KeyPressed(KeyPressedEvent{ Key: key, Mods: modifiers })).unwrap();
                }
                else {
                    self.EventBus.borrow_mut().add(Event::KeyReleased(KeyReleasedEvent{ Key: key })).unwrap();
                }
            },
            WindowEvent::MouseInput { state, button, modifiers, .. } => {
                if let winit::event::ElementState::Pressed = state {
                    self.EventBus.borrow_mut().add(Event::MousePressed(MouseButtonPressedEvent { MouseButton: button, Mods: modifiers })).unwrap();
                }
                else {
                    self.EventBus.borrow_mut().add(Event::MouseReleased(MouseButtonReleasedEvent { MouseButton: button })).unwrap();
                }
            },
            WindowEvent::MouseWheel { delta , .. } => {
                let d = if let winit::event::MouseScrollDelta::LineDelta(x, y) = delta{
                    (x, y)
                }
                else if let winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {x, y}) = delta{
                    (x as f32, y as f32)
                }
                else {return};
                self.EventBus.borrow_mut().add(Event::MouseScrolled(MouseScrollEvent { ScrollX: d.0, ScrollY: d.1})).unwrap();
            },
            WindowEvent::Resized(size) => {
                self.EventBus.borrow_mut().add(Event::WindowResize(WindowResizeEvent { Width: size.width, Height: size.height })).unwrap();
            },
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.EventBus.borrow_mut().add(Event::WindowResize(WindowResizeEvent { Width: new_inner_size.width, Height: new_inner_size.height })).unwrap();
            }
            WindowEvent::CloseRequested => {
                self.EventBus.borrow_mut().add(Event::WindowClose(WindowCloseEvent{})).unwrap();
            }
            _ => return
        }
    }

    fn HandleEvents(&mut self, controlFlow: &mut winit::event_loop::ControlFlow){
        //Handle all events within the event bus. Events triggered all over the application will be handled and propogated down here
        while self.EventBus.borrow().size() > 0 {
            let ev: Event = self.EventBus.borrow_mut().remove().unwrap();
            let mut dispatcher = EventDispatcher::New(&ev);

            //dispatch the window close event
            Dispatch!(Event::WindowClose(..), dispatcher, Self::OnWindowClose, self);
            
            //If the event was handled (aka the event was a window close event) or the excape key was pressed, exit the game
            if dispatcher.Handled {
                *controlFlow = winit::event_loop::ControlFlow::Exit;
                return;
            }
            else if let Event::KeyPressed(KeyPressedEvent {Key: winit::event::VirtualKeyCode::Escape, ..}) = ev {
                *controlFlow = winit::event_loop::ControlFlow::Exit;
                return;
            }

            //Handle window resize
            DispatchMut!(Event::WindowResize(..), dispatcher, Self::OnWindowResize, self);

            //Propogate down the event to the scenemanager
            if !dispatcher.Handled {
               self.SceneManager.OnEvent(&ev);
            }
        }
    }

    fn OnWindowResize(&mut self, event: &Event) -> bool{
        if let Event::WindowResize(WindowResizeEvent { Width, Height }) = event {
            self.Resize(*Width, *Height);

        }
        true
    }

    fn OnWindowClose(&self, _: &Event) -> bool{
        true
    }

    fn Resize(&mut self, width: u32, height: u32){
        if width > 0 && height > 0 {
            self.Wgpu.Config.width = width;
            self.Wgpu.Config.height = height;
            self.Wgpu.Surface.configure(&self.Wgpu.Device, &self.Wgpu.Config);
            unsafe { WINDOW_SIZE = (width, height) };
        }
    }

}
