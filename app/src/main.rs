struct App {}
impl engine::App for App {
    const FIXED_UPDATE_RATE: u32 = 100;
    fn fixed_update(&mut self, fixed_delta: f32) {
        println!("fixed update");
    }
    
    fn update(&mut self, delta: f32) {
        println!("update");
    }

    type RenderData = ();
    fn collect_render_data(&mut self) -> Self::RenderData {
        
    }
    
    fn render(data: Self::RenderData, delta: f32, renderer: std::sync::Arc<engine::Renderer>, scope: &rayon::Scope) {
        
    }
}

pub fn main() {
    let engine = engine::Engine::new(App {}).unwrap();
    engine.run().unwrap();
}