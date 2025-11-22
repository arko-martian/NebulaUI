//! # Nebula GFX - Universal Graphics Abstraction
//! 
//! The magic that makes Nebula UI work on EVERYTHING! 🌌
//! 
//! From cutting-edge GPUs to 20-year-old machines, Nebula GFX
//! automatically selects the best rendering backend available.
//! 
//! ## Backend Tiers:
//! - **Tier S (WebGPU)**: 2020+ hardware - Vulkan/Metal/DX12
//! - **Tier A (OpenGL 3.3)**: 2010+ hardware - Our primary target! (99% of users)
//! - **Tier B (OpenGL 2.1)**: 2005-2010 hardware - Legacy support
//! - **Tier C (CPU)**: Works on ANYTHING - Emergency fallback
//! 
//! ## Example:
//! ```rust,ignore
//! use nebula_gfx::{RendererBuilder, Backend};
//! 
//! // Automatic backend selection (recommended!)
//! let backend = RendererBuilder::new().select_backend();
//! 
//! // Or specify a preferred backend
//! let backend = RendererBuilder::new()
//!     .with_backend(Backend::OpenGL33)
//!     .select_backend();
//! ```

pub mod renderer;
pub mod backend;

pub use renderer::{Renderer, Color};
pub use backend::{Backend, RendererBuilder};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn exports_work() {
        // Just verify the exports are accessible
        let builder = RendererBuilder::new();
        let backend = builder.select_backend();
        
        // Should fall back to CPU in tests
        assert_eq!(backend, Backend::CPU);
    }
}
