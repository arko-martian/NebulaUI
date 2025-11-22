/// Universal renderer trait - works with ANY backend!
/// This is the magic that makes Nebula UI work on everything from
/// cutting-edge GPUs to 20-year-old machines! 🌌
pub trait Renderer {
    /// Color type used by this renderer
    type Color: Clone + Copy;
    
    /// Error type for this renderer
    type Error: std::error::Error;
    
    /// Set the clear color
    fn set_clear_color(&mut self, color: Self::Color);
    
    /// Resize the renderer
    fn resize(&mut self, width: u32, height: u32);
    
    /// Begin a new frame
    fn begin_frame(&mut self);
    
    /// Clear the screen
    fn clear(&mut self) -> Result<(), Self::Error>;
    
    /// End the current frame and present
    fn end_frame(&mut self) -> Result<(), Self::Error>;
    
    /// Get current dimensions
    fn dimensions(&self) -> (u32, u32);
    
    /// Get renderer name for debugging
    fn name(&self) -> &'static str;
}

/// Color trait - allows different color representations
pub trait Color: Clone + Copy + Sized {
    /// Create from RGB values
    fn rgb(r: u8, g: u8, b: u8) -> Self;
    
    /// Create from RGBA values
    fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self;
    
    /// Parse from hex string
    fn hex(hex: &str) -> Self;
    
    /// Nebula Blue - our signature color! 🌌
    const NEBULA_BLUE: Self;
    
    /// Common colors
    const BLACK: Self;
    const WHITE: Self;
    const RED: Self;
    const GREEN: Self;
    const BLUE: Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock color for testing
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct MockColor {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }
    
    impl Color for MockColor {
        fn rgb(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b, a: 255 }
        }
        
        fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self { r, g, b, a }
        }
        
        fn hex(_hex: &str) -> Self {
            Self::rgb(0, 0, 0)
        }
        
        const NEBULA_BLUE: Self = Self { r: 10, g: 14, b: 23, a: 255 };
        const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
        const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
        const RED: Self = Self { r: 255, g: 0, b: 0, a: 255 };
        const GREEN: Self = Self { r: 0, g: 255, b: 0, a: 255 };
        const BLUE: Self = Self { r: 0, g: 0, b: 255, a: 255 };
    }
    
    // Mock renderer for testing
    struct MockRenderer {
        width: u32,
        height: u32,
        clear_color: MockColor,
    }
    
    impl Renderer for MockRenderer {
        type Color = MockColor;
        type Error = std::io::Error;
        
        fn set_clear_color(&mut self, color: Self::Color) {
            self.clear_color = color;
        }
        
        fn resize(&mut self, width: u32, height: u32) {
            self.width = width;
            self.height = height;
        }
        
        fn begin_frame(&mut self) {}
        
        fn clear(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn end_frame(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        
        fn dimensions(&self) -> (u32, u32) {
            (self.width, self.height)
        }
        
        fn name(&self) -> &'static str {
            "Mock Renderer"
        }
    }
    
    #[test]
    fn renderer_trait_works() {
        let mut renderer = MockRenderer {
            width: 800,
            height: 600,
            clear_color: MockColor::NEBULA_BLUE,
        };
        
        assert_eq!(renderer.dimensions(), (800, 600));
        assert_eq!(renderer.name(), "Mock Renderer");
        
        renderer.resize(1024, 768);
        assert_eq!(renderer.dimensions(), (1024, 768));
        
        renderer.set_clear_color(MockColor::RED);
        assert_eq!(renderer.clear_color, MockColor::RED);
        
        assert!(renderer.clear().is_ok());
        assert!(renderer.end_frame().is_ok());
    }
    
    #[test]
    fn color_trait_works() {
        let color = MockColor::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
        
        let color = MockColor::rgba(255, 128, 64, 128);
        assert_eq!(color.a, 128);
        
        assert_eq!(MockColor::NEBULA_BLUE.r, 10);
        assert_eq!(MockColor::NEBULA_BLUE.g, 14);
        assert_eq!(MockColor::NEBULA_BLUE.b, 23);
    }
}
