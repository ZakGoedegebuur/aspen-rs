mod timing;

use hashbrown::HashMap;
use timing::Timer;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy}, window::{Window, WindowAttributes, WindowId}};

pub struct Engine {
    el: EventLoop<GlobalEvent>,
}

impl Engine {
    pub fn new() -> Result<Self, AppError> {
        let el = EventLoop::<GlobalEvent>::with_user_event().build().unwrap();
        Ok(Self {
            el,
        })
    }

    pub fn run(self) -> Result<AppExitInfo, AppError> {
        let mut app = App::new(&self.el).unwrap();
        self.el.run_app(&mut app).unwrap();
        Ok(AppExitInfo::NoInfo)
    }
}

#[derive(Debug)]
pub enum AppError {
}

#[derive(Debug)]
pub enum AppExitInfo {
    NoInfo
}

struct RenderContext {
    pub window: Window,
    pub recreate_swapchain: bool
}

#[derive(Debug)]
enum GlobalEvent {
    Update,
}

struct App {
    timer: Timer,
    event_loop_proxy: EventLoopProxy<GlobalEvent>,
    windows: HashMap<WindowId, RenderContext>,
}

impl App {
    pub fn new(event_loop: &EventLoop<GlobalEvent>) -> Result<Self, AppError> {
        Self {
            timer: Timer::new(),
            event_loop_proxy: event_loop.create_proxy(),
            windows: HashMap::new(),
        }
    }
}


impl ApplicationHandler<GlobalEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = event_loop.create_window(WindowAttributes::default().with_title("Default window")).unwrap();
        let bundle = RenderContext {
            window,
            recreate_swapchain: true,
        };
        self.windows.insert(bundle.window.id(), bundle);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {

        match event {
            WindowEvent::CloseRequested => {
                _ = self.windows.remove(&window_id);
                if self.windows.len() == 0 {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(_) => {
                let RenderContext { recreate_swapchain, .. } = self.windows.get_mut(&window_id).unwrap();
                *recreate_swapchain = true;
                //println!("window with title \"{}\" resized", window.title())
            }
            WindowEvent::RedrawRequested => {
                render();
            }
            _ => ()
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.event_loop_proxy.send_event(GlobalEvent::Update).unwrap();
        //rayon::scope(|s| {
        //    s.spawn(move |s| {
        //        println!("resolving shared data");
        //        sleep(Duration::from_millis(500));
        //        
        //        s.spawn(move |_| {
        //            println!("drawing screen 1");
        //            sleep(Duration::from_millis(500));
        //        });
        //        
        //        s.spawn(move |_| {
        //            println!("drawing screen 2");
        //            sleep(Duration::from_millis(500));
        //        });
        //    });
        //});
        //println!("finished rendering");
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GlobalEvent) {
        match event {
            GlobalEvent::Update => ()//println!("msg 2")
        }
    }
}