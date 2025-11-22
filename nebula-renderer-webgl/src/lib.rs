//! # Nebula WebGL Renderer 🌐
//! 
//! Brings Nebula UI to the WEB!
//! - WebGL 2.0 for modern browsers
//! - Canvas2D fallback for ancient browsers
//! - < 1 MB gzipped bundle size
//! - Works everywhere!

use tracing::{info, warn, error};

#[cfg(target_arch = "wasm32")]
pub mod webgl;

#[cfg(target_arch = "wasm32")]
pub mod canvas2d;

/// WebGL Renderer - Runs in the browser! 🌐
/// 
/// Features:
/// - WebGL 2.0 for modern browsers
/// - Canvas2D fallback for ancient browsers
/// - Automatic detection and fallback
/// - Optimized for small bundle size
#[derive(Debug)]
pub struct WebGLRenderer {
    /// Canvas element ID
    canvas_id: String,
    /// Renderer backend
    backend: RendererBackend,
    /// Canvas dimensions
    width: u32,
    height: u32,
}

/// Renderer backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererBackend {
    /// WebGL 2.0 (modern browsers)
    WebGL2,
    /// Canvas2D (fallback for ancient browsers)
    Canvas2D,
    /// Not initialized
    None,
}

impl WebGLRenderer {
    /// Create a new WebGL renderer
    pub fn new(canvas_id: impl Into<String>) -> Self {
        let canvas_id = canvas_id.into();
        info!("🌐 Creating WebGL Renderer for canvas: {}", canvas_id);
        
        Self {
            canvas_id,
            backend: RendererBackend::None,
            width: 800,
            height: 600,
        }
    }

    /// Initialize the renderer
    /// Tries WebGL 2.0 first, falls back to Canvas2D
    #[cfg(target_arch = "wasm32")]
    pub fn initialize(&mut self) -> Result<(), String> {
        info!("🚀 Initializing WebGL Renderer...");

        // Try WebGL 2.0 first
        match webgl::WebGL2Context::new(&self.canvas_id) {
            Ok(context) => {
                info!("✅ WebGL 2.0 initialized!");
                self.backend = RendererBackend::WebGL2;
                Ok(())
            }
            Err(e) => {
                warn!("⚠️ WebGL 2.0 failed: {}", e);
                warn!("🔄 Falling back to Canvas2D...");

                // Fallback to Canvas2D
                match canvas2d::Canvas2DContext::new(&self.canvas_id) {
                    Ok(context) => {
                        info!("✅ Canvas2D initialized!");
                        self.backend = RendererBackend::Canvas2D;
                        Ok(())
                    }
                    Err(e) => {
                        error!("❌ Canvas2D failed: {}", e);
                        Err(format!("Failed to initialize any renderer: {}", e))
                    }
                }
            }
        }
    }

    /// Initialize the renderer (non-WASM stub)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn initialize(&mut self) -> Result<(), String> {
        warn!("⚠️ WebGL renderer only works on WASM target");
        Err("WebGL renderer requires WASM target".to_string())
    }

    /// Get current backend
    pub fn backend(&self) -> RendererBackend {
        self.backend
    }

    /// Set canvas size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        info!("📐 Canvas size set to {}x{}", width, height);
    }

    /// Get canvas size
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Clear the canvas
    #[cfg(target_arch = "wasm32")]
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) -> Result<(), String> {
        match self.backend {
            RendererBackend::WebGL2 => {
                webgl::clear(&self.canvas_id, r, g, b, a)
            }
            RendererBackend::Canvas2D => {
                canvas2d::clear(&self.canvas_id, r, g, b, a)
            }
            RendererBackend::None => {
                Err("Renderer not initialized".to_string())
            }
        }
    }

    /// Clear the canvas (non-WASM stub)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn clear(&self, _r: f32, _g: f32, _b: f32, _a: f32) -> Result<(), String> {
        Err("WebGL renderer requires WASM target".to_string())
    }

    /// Present the frame
    pub fn present(&self) {
        // WebGL/Canvas2D automatically presents
        // This is here for API compatibility
    }
}

impl Default for WebGLRenderer {
    fn default() -> Self {
        Self::new("canvas")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renderer_creation() {
        let renderer = WebGLRenderer::new("test-canvas");
        assert_eq!(renderer.canvas_id, "test-canvas");
        assert_eq!(renderer.backend, RendererBackend::None);
    }

    #[test]
    fn renderer_default() {
        let renderer = WebGLRenderer::default();
        assert_eq!(renderer.canvas_id, "canvas");
    }

    #[test]
    fn renderer_set_size() {
        let mut renderer = WebGLRenderer::new("test");
        renderer.set_size(1920, 1080);
        assert_eq!(renderer.size(), (1920, 1080));
    }

    #[test]
    fn renderer_backend() {
        let renderer = WebGLRenderer::new("test");
        assert_eq!(renderer.backend(), RendererBackend::None);
    }

    #[test]
    fn backend_equality() {
        assert_eq!(RendererBackend::WebGL2, RendererBackend::WebGL2);
        assert_ne!(RendererBackend::WebGL2, RendererBackend::Canvas2D);
        assert_ne!(RendererBackend::Canvas2D, RendererBackend::None);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn initialize_fails_on_non_wasm() {
        let mut renderer = WebGLRenderer::new("test");
        let result = renderer.initialize();
        assert!(result.is_err());
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn clear_fails_on_non_wasm() {
        let renderer = WebGLRenderer::new("test");
        let result = renderer.clear(0.0, 0.0, 0.0, 1.0);
        assert!(result.is_err());
    }
}
