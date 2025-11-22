use tracing::{info, warn};

/// Rendering backend tier system
/// Nebula UI automatically selects the best available backend!
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Tier S: WebGPU (Future - cutting edge, 2020+ hardware)
    /// - Vulkan, Metal, DX12 backends
    /// - Compute shaders, ray tracing
    /// - Target: 2020+ hardware
    WebGPU,
    
    /// Tier A: OpenGL 3.3 (Standard - our primary target!)
    /// - Modern OpenGL with shaders
    /// - Vertex buffers, textures, FBOs
    /// - Target: 2010+ hardware (99% of users)
    OpenGL33,
    
    /// Tier B: OpenGL 2.1 (Legacy - for old machines)
    /// - Fixed function pipeline
    /// - Basic shaders
    /// - Target: 2005-2010 hardware
    OpenGL21,
    
    /// Tier C: CPU Rendering (Emergency fallback)
    /// - Pure software rendering
    /// - Works on ANYTHING
    /// - Target: When all else fails!
    CPU,
}

impl Backend {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Backend::WebGPU => "WebGPU (Tier S - Cutting Edge)",
            Backend::OpenGL33 => "OpenGL 3.3 (Tier A - Standard)",
            Backend::OpenGL21 => "OpenGL 2.1 (Tier B - Legacy)",
            Backend::CPU => "CPU (Tier C - Emergency Fallback)",
        }
    }
    
    /// Get tier level (higher is better)
    pub fn tier(&self) -> u8 {
        match self {
            Backend::WebGPU => 4,
            Backend::OpenGL33 => 3,
            Backend::OpenGL21 => 2,
            Backend::CPU => 1,
        }
    }
}

/// Renderer builder with automatic backend selection
/// This is the MAGIC that makes Nebula UI work everywhere! 🌌
pub struct RendererBuilder {
    preferred_backend: Option<Backend>,
    fallback_chain: Vec<Backend>,
}

impl RendererBuilder {
    /// Create a new renderer builder
    pub fn new() -> Self {
        Self {
            preferred_backend: None,
            fallback_chain: vec![
                Backend::OpenGL33,  // Try OpenGL 3.3 first (our primary target)
                Backend::CPU,       // Fall back to CPU if OpenGL fails
            ],
        }
    }
    
    /// Set preferred backend
    pub fn with_backend(mut self, backend: Backend) -> Self {
        self.preferred_backend = Some(backend);
        self
    }
    
    /// Set custom fallback chain
    pub fn with_fallback_chain(mut self, chain: Vec<Backend>) -> Self {
        self.fallback_chain = chain;
        self
    }
    
    /// Select the best available backend
    /// This tries backends in order until one works!
    pub fn select_backend(&self) -> Backend {
        info!("🎨 Selecting rendering backend...");
        
        // If user specified a backend, try that first
        if let Some(preferred) = self.preferred_backend {
            info!("User requested: {}", preferred.name());
            if self.is_backend_available(preferred) {
                info!("✅ Using preferred backend: {}", preferred.name());
                return preferred;
            } else {
                warn!("❌ Preferred backend not available, trying fallbacks...");
            }
        }
        
        // Try fallback chain
        for backend in &self.fallback_chain {
            info!("Trying: {}", backend.name());
            if self.is_backend_available(*backend) {
                info!("✅ Selected backend: {}", backend.name());
                return *backend;
            }
            warn!("❌ {} not available", backend.name());
        }
        
        // Ultimate fallback - CPU always works!
        warn!("⚠️  All backends failed, using CPU fallback");
        Backend::CPU
    }
    
    /// Check if a backend is available
    /// For now, this is a simple check - in production, we'd probe the system
    fn is_backend_available(&self, backend: Backend) -> bool {
        match backend {
            Backend::WebGPU => {
                // WebGPU not implemented yet
                false
            }
            Backend::OpenGL33 => {
                // For now, assume OpenGL 3.3 is available
                // In production, we'd check with glutin/winit
                // For this demo, we'll say it's NOT available to test fallback
                false
            }
            Backend::OpenGL21 => {
                // OpenGL 2.1 not implemented yet
                false
            }
            Backend::CPU => {
                // CPU always works!
                true
            }
        }
    }
}

impl Default for RendererBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn backend_names() {
        assert_eq!(Backend::WebGPU.name(), "WebGPU (Tier S - Cutting Edge)");
        assert_eq!(Backend::OpenGL33.name(), "OpenGL 3.3 (Tier A - Standard)");
        assert_eq!(Backend::OpenGL21.name(), "OpenGL 2.1 (Tier B - Legacy)");
        assert_eq!(Backend::CPU.name(), "CPU (Tier C - Emergency Fallback)");
    }
    
    #[test]
    fn backend_tiers() {
        assert_eq!(Backend::WebGPU.tier(), 4);
        assert_eq!(Backend::OpenGL33.tier(), 3);
        assert_eq!(Backend::OpenGL21.tier(), 2);
        assert_eq!(Backend::CPU.tier(), 1);
    }
    
    #[test]
    fn builder_default() {
        let builder = RendererBuilder::new();
        assert_eq!(builder.preferred_backend, None);
        assert_eq!(builder.fallback_chain.len(), 2);
    }
    
    #[test]
    fn builder_with_backend() {
        let builder = RendererBuilder::new().with_backend(Backend::OpenGL33);
        assert_eq!(builder.preferred_backend, Some(Backend::OpenGL33));
    }
    
    #[test]
    fn builder_selects_fallback() {
        let builder = RendererBuilder::new();
        let backend = builder.select_backend();
        
        // Should fall back to CPU since OpenGL is not available in tests
        assert_eq!(backend, Backend::CPU);
    }
    
    #[test]
    fn builder_custom_fallback_chain() {
        let builder = RendererBuilder::new()
            .with_fallback_chain(vec![Backend::WebGPU, Backend::CPU]);
        
        assert_eq!(builder.fallback_chain.len(), 2);
        assert_eq!(builder.fallback_chain[0], Backend::WebGPU);
        assert_eq!(builder.fallback_chain[1], Backend::CPU);
    }
    
    #[test]
    fn backend_equality() {
        assert_eq!(Backend::CPU, Backend::CPU);
        assert_ne!(Backend::CPU, Backend::OpenGL33);
    }
}
