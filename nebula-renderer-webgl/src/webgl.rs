//! WebGL 2.0 Context - Modern browser rendering! 🚀

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use tracing::{info, error};

/// WebGL 2.0 Context
pub struct WebGL2Context {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
}

impl WebGL2Context {
    /// Create a new WebGL 2.0 context
    pub fn new(canvas_id: &str) -> Result<Self, String> {
        info!("🎨 Creating WebGL 2.0 context for: {}", canvas_id);

        // Get the window
        let window = web_sys::window()
            .ok_or_else(|| "No window object".to_string())?;

        // Get the document
        let document = window.document()
            .ok_or_else(|| "No document object".to_string())?;

        // Get the canvas element
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| format!("Canvas element '{}' not found", canvas_id))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas".to_string())?;

        // Get WebGL 2.0 context
        let gl = canvas
            .get_context("webgl2")
            .map_err(|_| "Failed to get WebGL2 context".to_string())?
            .ok_or_else(|| "WebGL2 context is null".to_string())?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| "Failed to cast to WebGL2 context".to_string())?;

        info!("✅ WebGL 2.0 context created!");

        Ok(Self { canvas, gl })
    }

    /// Get the WebGL context
    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    /// Get the canvas
    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    /// Clear the canvas
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    /// Set viewport
    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.gl.viewport(x, y, width, height);
    }
}

/// Clear the canvas (convenience function)
pub fn clear(canvas_id: &str, r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
    let context = WebGL2Context::new(canvas_id)?;
    context.clear(r, g, b, a);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webgl_context_creation_fails_without_dom() {
        // This will fail in test environment (no DOM)
        let result = WebGL2Context::new("test-canvas");
        assert!(result.is_err());
    }

    #[test]
    fn clear_fails_without_dom() {
        let result = clear("test-canvas", 0.0, 0.0, 0.0, 1.0);
        assert!(result.is_err());
    }
}
