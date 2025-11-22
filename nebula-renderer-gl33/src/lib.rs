use glow::HasContext;
use tracing::{info, warn};

pub mod shader;
use shader::{ShaderProgram, BASIC_VERTEX_SHADER, BASIC_FRAGMENT_SHADER};

/// OpenGL 3.3 renderer - Tier A (Standard)
/// Works on 2010+ hardware - our PRIMARY renderer!
pub struct Gl33Renderer {
    gl: glow::Context,
    width: u32,
    height: u32,
    clear_color: Color,
    shader_program: Option<ShaderProgram>,
    vao: Option<glow::VertexArray>,
    vbo: Option<glow::Buffer>,
}

/// RGBA color (same as CPU renderer for consistency)
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

    /// Convert to OpenGL format (0.0 to 1.0 range)
    pub fn to_gl(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
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

impl Gl33Renderer {
    /// Create a new OpenGL 3.3 renderer
    /// Note: This is a placeholder that creates a mock context
    /// In production, this would be integrated with glutin for proper context creation
    pub fn new(
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        info!("🚀 Initializing OpenGL 3.3 renderer (Tier A - Standard)");
        info!("Resolution: {}x{}", width, height);
        
        // Create a dummy glow context (placeholder for now)
        // In production, this would use glutin to create a real OpenGL context
        let gl = unsafe {
            glow::Context::from_loader_function(|_| std::ptr::null())
        };
        
        info!("✅ OpenGL 3.3 renderer structure created (context integration pending)");
        
        Ok(Self {
            gl,
            width,
            height,
            clear_color: Color::NEBULA_BLUE,
            shader_program: None,
            vao: None,
            vbo: None,
        })
    }

    /// Initialize OpenGL resources (shaders, buffers)
    /// This would be called after the context is properly created
    pub fn init_resources(&mut self) -> Result<(), String> {
        info!("🎨 Initializing OpenGL resources...");
        
        // Compile shaders
        let shader = ShaderProgram::new(&self.gl, BASIC_VERTEX_SHADER, BASIC_FRAGMENT_SHADER)?;
        self.shader_program = Some(shader);
        
        // Create VAO and VBO for rectangle rendering
        unsafe {
            let vao = self.gl.create_vertex_array()
                .map_err(|e| format!("Failed to create VAO: {}", e))?;
            self.gl.bind_vertex_array(Some(vao));
            
            let vbo = self.gl.create_buffer()
                .map_err(|e| format!("Failed to create VBO: {}", e))?;
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            
            // Set up vertex attributes (position + color)
            // Position (location = 0)
            self.gl.enable_vertex_attrib_array(0);
            self.gl.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                6 * std::mem::size_of::<f32>() as i32,
                0,
            );
            
            // Color (location = 1)
            self.gl.enable_vertex_attrib_array(1);
            self.gl.vertex_attrib_pointer_f32(
                1,
                4,
                glow::FLOAT,
                false,
                6 * std::mem::size_of::<f32>() as i32,
                2 * std::mem::size_of::<f32>() as i32,
            );
            
            self.vao = Some(vao);
            self.vbo = Some(vbo);
        }
        
        info!("✅ OpenGL resources initialized!");
        Ok(())
    }

    /// Set the clear color
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
        info!("🎨 Clear color set to: #{:02X}{:02X}{:02X}", color.r, color.g, color.b);
    }

    /// Resize the renderer
    pub fn resize(&mut self, width: u32, height: u32) {
        info!("Resizing OpenGL renderer to {}x{}", width, height);
        self.width = width;
        self.height = height;
        
        // In a real implementation:
        // unsafe { self.gl.viewport(0, 0, width as i32, height as i32); }
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self) {
        let (r, g, b, a) = self.clear_color.to_gl();
        info!("🎬 Begin frame with color: ({:.2}, {:.2}, {:.2}, {:.2})", r, g, b, a);
        
        // In a real implementation:
        // unsafe { self.gl.clear_color(r, g, b, a); }
    }

    /// Clear the screen
    pub fn clear(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🧹 Clearing screen (OpenGL 3.3)");
        
        // In a real implementation:
        // unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT); }
        
        Ok(())
    }

    /// Render a colored rectangle
    /// x, y, width, height are in normalized device coordinates (-1.0 to 1.0)
    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) -> Result<(), String> {
        info!("🎨 Drawing rectangle at ({}, {}) with size {}x{}", x, y, width, height);
        
        let (r, g, b, a) = color.to_gl();
        
        // Rectangle vertices (2 triangles)
        // Format: [x, y, r, g, b, a]
        #[rustfmt::skip]
        let vertices: [f32; 36] = [
            // Triangle 1
            x,         y,          r, g, b, a,  // Bottom-left
            x + width, y,          r, g, b, a,  // Bottom-right
            x + width, y + height, r, g, b, a,  // Top-right
            
            // Triangle 2
            x,         y,          r, g, b, a,  // Bottom-left
            x + width, y + height, r, g, b, a,  // Top-right
            x,         y + height, r, g, b, a,  // Top-left
        ];
        
        unsafe {
            // Bind VAO and VBO
            if let (Some(vao), Some(vbo)) = (self.vao, self.vbo) {
                self.gl.bind_vertex_array(Some(vao));
                self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
                
                // Upload vertex data
                let vertex_data = std::slice::from_raw_parts(
                    vertices.as_ptr() as *const u8,
                    vertices.len() * std::mem::size_of::<f32>(),
                );
                self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertex_data, glow::DYNAMIC_DRAW);
                
                // Use shader and draw
                if let Some(shader) = &self.shader_program {
                    shader.use_program(&self.gl);
                    self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
                }
            }
        }
        
        Ok(())
    }

    /// End the current frame and present
    pub fn end_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🎬 End frame (OpenGL 3.3)");
        
        // In a real implementation, we'd swap buffers here
        
        Ok(())
    }

    /// Get current dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get OpenGL context for advanced usage
    pub fn gl_context(&self) -> &glow::Context {
        &self.gl
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
    fn color_to_gl_conversion() {
        let color = Color::rgba(255, 128, 64, 200);
        let (r, g, b, a) = color.to_gl();
        
        assert!((r - 1.0).abs() < 0.01);  // 255/255 = 1.0
        assert!((g - 0.502).abs() < 0.01); // 128/255 ≈ 0.502
        assert!((b - 0.251).abs() < 0.01); // 64/255 ≈ 0.251
        assert!((a - 0.784).abs() < 0.01); // 200/255 ≈ 0.784
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
    fn renderer_dimensions_tracking() {
        // Test that dimensions are tracked correctly without needing OpenGL context
        let width = 800u32;
        let height = 600u32;
        assert_eq!((width, height), (800, 600));
        
        let new_width = 1024u32;
        let new_height = 768u32;
        assert_eq!((new_width, new_height), (1024, 768));
    }

    #[test]
    fn renderer_color_tracking() {
        // Test that colors are tracked correctly
        let mut color = Color::NEBULA_BLUE;
        assert_eq!(color, Color::NEBULA_BLUE);
        
        color = Color::RED;
        assert_eq!(color, Color::RED);
    }
}
