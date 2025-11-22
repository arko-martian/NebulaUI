//! Canvas2D Context - Fallback for ancient browsers! 🦕

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use tracing::{info, error};

/// Canvas2D Context
pub struct Canvas2DContext {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Canvas2DContext {
    /// Create a new Canvas2D context
    pub fn new(canvas_id: &str) -> Result<Self, String> {
        info!("🎨 Creating Canvas2D context for: {}", canvas_id);

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

        // Get 2D context
        let ctx = canvas
            .get_context("2d")
            .map_err(|_| "Failed to get 2D context".to_string())?
            .ok_or_else(|| "2D context is null".to_string())?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| "Failed to cast to 2D context".to_string())?;

        info!("✅ Canvas2D context created!");

        Ok(Self { canvas, ctx })
    }

    /// Get the 2D context
    pub fn ctx(&self) -> &CanvasRenderingContext2d {
        &self.ctx
    }

    /// Get the canvas
    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    /// Clear the canvas
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        // Convert to CSS color
        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;
        let color = format!("rgba({}, {}, {}, {})", r, g, b, a);

        self.ctx.set_fill_style(&color.into());
        self.ctx.fill_rect(0.0, 0.0, width, height);
    }

    /// Draw a rectangle
    pub fn draw_rect(&self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        self.ctx.set_fill_style(&color.into());
        self.ctx.fill_rect(x, y, width, height);
    }

    /// Draw text
    pub fn draw_text(&self, text: &str, x: f64, y: f64, color: &str) {
        self.ctx.set_fill_style(&color.into());
        let _ = self.ctx.fill_text(text, x, y);
    }
}

/// Clear the canvas (convenience function)
pub fn clear(canvas_id: &str, r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
    let context = Canvas2DContext::new(canvas_id)?;
    context.clear(r, g, b, a);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas2d_context_creation_fails_without_dom() {
        // This will fail in test environment (no DOM)
        let result = Canvas2DContext::new("test-canvas");
        assert!(result.is_err());
    }

    #[test]
    fn clear_fails_without_dom() {
        let result = clear("test-canvas", 0.0, 0.0, 0.0, 1.0);
        assert!(result.is_err());
    }
}
