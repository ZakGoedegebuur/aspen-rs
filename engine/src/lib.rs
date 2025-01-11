mod timing;

use std::{error::Error, sync::Arc};

pub use renderer::Renderer;
pub use timing::Timer;
use winit::{
    application::ApplicationHandler, 
    event::WindowEvent, 
    event_loop::{
        ActiveEventLoop, 
        ControlFlow, 
        EventLoop, 
    }, 
    window::{
        Window, 
        WindowAttributes, 
        WindowId
    }
};

pub trait App {
    const FIXED_UPDATE_RATE: u32;
    fn fixed_update(&mut self, fixed_delta: f32);
    fn update(&mut self, delta: f32);

    type RenderData: Send;
    fn collect_render_data(&mut self) -> Self::RenderData;
    fn render(data: Self::RenderData, delta: f32, renderer: std::sync::Arc<crate::Renderer>, scope: &rayon::Scope);
}

#[derive(Debug)]
pub struct Engine<A> {
    el: EventLoop<CustomEvent>,
    inner: EngineInner<A>
}

impl<A: App + Send> Engine<A> {
    pub fn new(app: A) -> Result<Self, Box<dyn Error>> {
        let el = EventLoop::with_user_event().build()?;

        let inner = EngineInner {
            app,
            timer: Timer::new(),
            #[cfg(feature = "rendering")]
            renderer: Arc::new(Renderer { })
        };

        Ok(Self {
            el,
            inner
        })
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.el.run_app(&mut self.inner)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct EngineInner<A> {
    app: A,
    timer: Timer,
    #[cfg(feature = "rendering")]
    renderer: Arc<Renderer>,
}

impl<A: App + Send> ApplicationHandler<CustomEvent> for EngineInner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
        
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        
        #[cfg(feature = "rendering")]
        {
            rayon::scope(|s| {
                let timing = self.timer.update(A::FIXED_UPDATE_RATE);

                let data = self.app.collect_render_data();
                let renderer = self.renderer.clone();
                s.spawn(move |s| {
                    A::render(data, timing.delta, renderer, s);
                    //println!("resolving shared data");
                    //s.spawn(move |_| {
                    //    println!("drawing screen 1");
                    //});
                    //
                    //s.spawn(move |_| {
                    //    println!("drawing screen 2");
                    //});
                });

                for _ in 0..timing.fixed_steps {
                    self.app.fixed_update(timing.fixed_delta);
                }
                
                self.app.update(timing.delta);
            });
        }
        
        #[cfg(not(feature = "rendering"))]
        {
            let timing = self.timer.update(A::FIXED_UPDATE_RATE);
            for _ in 0..timing.fixed_steps {
                self.app.fixed_update(timing.fixed_delta);
            }
            
            self.app.update(timing.delta);
        }
    }
}

pub enum CustomEvent { }