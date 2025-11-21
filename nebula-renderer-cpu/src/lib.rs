use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use tracing::{info, warn};

/// CPU-based renderer using softbuffer + tiny-skia
/// This is Tier C - the emergency fallback that ALWAYS works
pub struct CpuRenderer<D, W> {
    context: Context<D>,
    surface: Surface<D, W>,
    width: u32,
    height: u32,
    clear_color: Color,
}

/// RGBA color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create color from RGB values
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create color from RGBA values
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Parse hex color (#RRGGBB or #RRGGBBAA)
    pub fn hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                Self::rgb(r, g, b)
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
                Self::rgba(r, g, b, a)
            }
            _ => {
                warn!("Invalid hex color: {}, using black", hex);
                Self::rgb(0, 0, 0)
            }
        }
    }

    /// Convert to u32 in ARGB format (for softbuffer)
    pub fn to_argb(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Named colors
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    
    // Nebula Blue! 🌌
    pub const NEBULA_BLUE: Self = Self::rgb(10, 14, 23);
}

impl<D, W> CpuRenderer<D, W>
where
    D: raw_window_handle::HasDisplayHandle,
    W: raw_window_handle::HasWindowHandle,
{
    /// Create a new CPU renderer
    pub fn new(
        display_handle: D,
        window_handle: W,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing CPU renderer (Tier C - Emergency Fallback)");
        info!("Resolution: {}x{}", width, height);

        let context = Context::new(display_handle)?;
        let surface = Surface::new(&context, window_handle)?;

        Ok(Self {
            context,
            surface,
            width,
            height,
            clear_color: Color::NEBULA_BLUE,
        })
    }

    /// Set the clear color
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    /// Resize the renderer
    pub fn resize(&mut self, width: u32, height: u32) {
        info!("Resizing CPU renderer to {}x{}", width, height);
        self.width = width;
        self.height = height;
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self) {
        // Nothing to do for CPU renderer
    }

    /// Clear the screen with the current clear color
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let width = NonZeroU32::new(self.width).ok_or("Width is zero")?;
        let height = NonZeroU32::new(self.height).ok_or("Height is zero")?;

        self.surface.resize(width, height)?;

        let mut buffer = self.surface.buffer_mut()?;
        let clear_color = self.clear_color.to_argb();

        // Fill the entire buffer with the clear color
        for pixel in buffer.iter_mut() {
            *pixel = clear_color;
        }

        buffer.present()?;

        Ok(())
    }

    /// End the current frame and present
    pub fn end_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // For CPU renderer, we present in clear() for now
        // In the future, we'll accumulate draw calls and present here
        Ok(())
    }

    /// Get current dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_rgb_creates_opaque_color() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn color_rgba_creates_transparent_color() {
        let color = Color::rgba(255, 128, 64, 128);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn color_hex_parses_6_digit() {
        let color = Color::hex("#FF8040");
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn color_hex_parses_8_digit() {
        let color = Color::hex("#FF804080");
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn color_hex_handles_no_hash() {
        let color = Color::hex("FF8040");
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn nebula_blue_is_correct() {
        let color = Color::NEBULA_BLUE;
        assert_eq!(color.r, 10);
        assert_eq!(color.g, 14);
        assert_eq!(color.b, 23);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn color_to_argb_conversion() {
        let color = Color::rgba(255, 128, 64, 200);
        let argb = color.to_argb();
        
        // ARGB format: 0xAARRGGBB
        assert_eq!(argb, 0xC8FF8040);
    }
}
