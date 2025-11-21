use nebula_platform::{InputHandler, Key, MouseButtonEvent, MousePosition, NebulaWindow, RenderCallback};
use nebula_renderer_cpu::{Color, CpuRenderer};
use std::sync::{Arc, Mutex};
use tracing_subscriber;
use winit::window::Window;

// Helper struct to hold window handles with 'static lifetime
// This is safe because the window lives for the entire duration of the app
struct WindowHandles {
    window_ptr: *const Window,
}

unsafe impl Send for WindowHandles {}
unsafe impl Sync for WindowHandles {}

impl WindowHandles {
    fn new(window: &Window) -> Self {
        Self {
            window_ptr: window as *const Window,
        }
    }
    
    unsafe fn get(&self) -> &'static Window {
        &*self.window_ptr
    }
}

struct App {
    renderer: Arc<Mutex<Option<CpuRenderer<&'static Window, &'static Window>>>>,
    handles: Option<WindowHandles>,
}

impl App {
    fn new() -> Self {
        Self {
            renderer: Arc::new(Mutex::new(None)),
            handles: None,
        }
    }
}

impl InputHandler for App {
    fn on_mouse_down(&mut self, button: MouseButtonEvent, position: MousePosition) {
        tracing::info!("🖱️  Clicked! Button: {:?} at ({:.0}, {:.0})", button, position.x, position.y);
    }
    
    fn on_key_down(&mut self, key: Key) {
        tracing::info!("⌨️  Key pressed: {:?}", key);
        
        // Exit on Escape key
        if key == Key::Escape {
            tracing::info!("Escape pressed, exiting...");
            std::process::exit(0);
        }
    }
}

impl RenderCallback for App {
    fn render(&mut self, window: &Window) {
        // Initialize renderer on first render
        if self.handles.is_none() {
            self.handles = Some(WindowHandles::new(window));
            
            let size = window.inner_size();
            
            unsafe {
                let window_ref = self.handles.as_ref().unwrap().get();
                
                match CpuRenderer::new(
                    window_ref,
                    window_ref,
                    size.width,
                    size.height,
                ) {
                    Ok(mut renderer) => {
                        // Set the clear color to Nebula Blue! 🌌
                        renderer.set_clear_color(Color::NEBULA_BLUE);
                        *self.renderer.lock().unwrap() = Some(renderer);
                        tracing::info!("🌌 CPU Renderer initialized successfully!");
                        tracing::info!("Rendering Nebula Blue (#0A0E17)!");
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize CPU renderer: {}", e);
                        return;
                    }
                }
            }
        }

        // Render frame
        if let Some(renderer) = self.renderer.lock().unwrap().as_mut() {
            renderer.begin_frame();
            
            if let Err(e) = renderer.clear() {
                tracing::error!("Failed to clear: {}", e);
            }
            
            if let Err(e) = renderer.end_frame() {
                tracing::error!("Failed to end frame: {}", e);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("🌌 Nebula UI - Tracer Bullet 🚀");
    tracing::info!("Rendering with CPU (Tier C - Emergency Fallback)");

    // Create app
    let app = App::new();

    // Create and run window
    let window = NebulaWindow::new("Nebula UI - Tracer Bullet", 800, 600)
        .with_render_callback(app);
    
    window.run()?;

    Ok(())
}
