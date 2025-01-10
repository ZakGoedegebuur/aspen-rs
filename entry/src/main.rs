use std::{thread::sleep, time::{Duration, Instant}};

use hashbrown::HashMap;
use timing::Timer;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy}, window::{Window, WindowAttributes, WindowId}};

mod timing;

pub fn main() {
    let mut timer = Timer::new();
    let event_loop = EventLoop::<GlobalEvent>::with_user_event().build().unwrap();

    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap();

    //loop {
    //    match timer.update(100) {
    //        t if t.fixed_steps > 0 => println!("{:#?}", t),
    //        _ => (),
    //    }
    //}
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
    pub fn new(event_loop: &EventLoop<GlobalEvent>) -> Self {
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

fn fixed_update() {

}

fn update() {

}

fn render() {
    println!("render() called")
}