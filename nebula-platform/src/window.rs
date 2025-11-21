use crate::input::{is_key_pressed, is_key_released, key_from_event, InputHandler, MouseButtonEvent, MousePosition};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

/// Callback trait for rendering and input
pub trait RenderCallback: InputHandler {
    fn render(&mut self, window: &Window);
}

/// Window manager for Nebula UI
pub struct NebulaWindow<R: RenderCallback> {
    window: Option<Window>,
    title: String,
    width: u32,
    height: u32,
    render_callback: Option<R>,
    mouse_position: MousePosition,
}

impl<R: RenderCallback> NebulaWindow<R> {
    /// Create a new window configuration
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            window: None,
            title: title.into(),
            width,
            height,
            render_callback: None,
            mouse_position: MousePosition::new(0.0, 0.0),
        }
    }

    /// Set the render callback
    pub fn with_render_callback(mut self, callback: R) -> Self {
        self.render_callback = Some(callback);
        self
    }

    /// Get a reference to the window
    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    /// Run the event loop
    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting Nebula UI window: {}", self.title);
        
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        
        event_loop.run_app(&mut self)?;
        
        Ok(())
    }
}

impl<R: RenderCallback> ApplicationHandler for NebulaWindow<R> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title(&self.title)
                .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));
            
            match event_loop.create_window(window_attributes) {
                Ok(window) => {
                    tracing::info!("Window created successfully");
                    self.window = Some(window);
                }
                Err(e) => {
                    tracing::error!("Failed to create window: {}", e);
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                tracing::info!("Close requested, exiting");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Call the render callback
                if let (Some(window), Some(callback)) = (&self.window, &mut self.render_callback) {
                    callback.render(window);
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                tracing::info!("Window resized to {}x{}", size.width, size.height);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(callback) = &mut self.render_callback {
                    let button_event = MouseButtonEvent::from(button);
                    match state {
                        ElementState::Pressed => {
                            tracing::info!("Mouse button pressed: {:?} at ({}, {})", button_event, self.mouse_position.x, self.mouse_position.y);
                            callback.on_mouse_down(button_event, self.mouse_position);
                        }
                        ElementState::Released => {
                            tracing::info!("Mouse button released: {:?} at ({}, {})", button_event, self.mouse_position.x, self.mouse_position.y);
                            callback.on_mouse_up(button_event, self.mouse_position);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = MousePosition::new(position.x, position.y);
                if let Some(callback) = &mut self.render_callback {
                    callback.on_mouse_move(self.mouse_position);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(callback) = &mut self.render_callback {
                    if let Some(key) = key_from_event(&event) {
                        if is_key_pressed(&event) {
                            tracing::info!("Key pressed: {:?}", key);
                            callback.on_key_down(key);
                        } else if is_key_released(&event) {
                            tracing::info!("Key released: {:?}", key);
                            callback.on_key_up(key);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
