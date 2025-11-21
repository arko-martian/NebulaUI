use nebula_core::Signal;
use nebula_platform::{
    InputHandler, Key, MouseButtonEvent, MousePosition, NebulaWindow, RenderCallback,
};
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
    // 🌟 SIGNAL! The reactive heart of Nebula UI!
    background_color: Signal<Color>,
    color_index: usize,
}

impl App {
    fn new() -> Self {
        // Create a signal for background color
        let background_color = Signal::new(Color::NEBULA_BLUE);

        Self {
            renderer: Arc::new(Mutex::new(None)),
            handles: None,
            background_color,
            color_index: 0,
        }
    }

    fn cycle_color(&mut self) {
        // Cycle through 3 beautiful colors! 🎨
        let colors = [
            Color::NEBULA_BLUE,      // #0A0E17 - Our signature! 🌌
            Color::hex("#8338ec"),   // Purple - From Dark Matter theme! 💜
            Color::hex("#ff006e"),   // Pink - Cosmic gradient! 💖
        ];

        self.color_index = (self.color_index + 1) % colors.len();
        let new_color = colors[self.color_index];

        tracing::info!(
            "🎨 Color changed to: #{:02X}{:02X}{:02X}",
            new_color.r,
            new_color.g,
            new_color.b
        );

        // Update the signal! This will notify all subscribers!
        self.background_color.set(new_color);
    }
}

impl InputHandler for App {
    fn on_mouse_down(&mut self, button: MouseButtonEvent, position: MousePosition) {
        tracing::info!(
            "🖱️  Clicked! Button: {:?} at ({:.0}, {:.0})",
            button,
            position.x,
            position.y
        );

        // Cycle color on left click!
        if button == MouseButtonEvent::Left {
            self.cycle_color();
        }
    }

    fn on_key_down(&mut self, key: Key) {
        tracing::info!("⌨️  Key pressed: {:?}", key);

        // Exit on Escape key
        if key == Key::Escape {
            tracing::info!("Escape pressed, exiting...");
            std::process::exit(0);
        }

        // Cycle color on Space key too!
        if key == Key::Space {
            self.cycle_color();
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

                match CpuRenderer::new(window_ref, window_ref, size.width, size.height) {
                    Ok(mut renderer) => {
                        // Set initial color from signal! 🌟
                        renderer.set_clear_color(self.background_color.get());
                        *self.renderer.lock().unwrap() = Some(renderer);
                        tracing::info!("🌌 CPU Renderer initialized successfully!");
                        tracing::info!("🎨 Initial color: Nebula Blue (#0A0E17)!");
                        tracing::info!("💡 Click or press Space to cycle colors!");
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize CPU renderer: {}", e);
                        return;
                    }
                }
            }
        }

        // Render frame with current signal value
        if let Some(renderer) = self.renderer.lock().unwrap().as_mut() {
            // Update renderer color from signal
            renderer.set_clear_color(self.background_color.get());

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
    tracing::info!("⚡ Quantum Signals - Reactive Magic!");
    tracing::info!("Rendering with CPU (Tier C - Emergency Fallback)");

    // Create app
    let app = App::new();

    // Create and run window
    let window = NebulaWindow::new("Nebula UI - Tracer Bullet", 800, 600).with_render_callback(app);

    window.run()?;

    Ok(())
}
