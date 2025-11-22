use glow::HasContext;
use tracing::info;

/// Shader program wrapper
pub struct ShaderProgram {
    pub program: glow::Program,
}

impl ShaderProgram {
    /// Create a new shader program from vertex and fragment shader source
    pub fn new(
        gl: &glow::Context,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<Self, String> {
        unsafe {
            // Compile vertex shader
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER)
                .map_err(|e| format!("Failed to create vertex shader: {}", e))?;
            
            gl.shader_source(vertex_shader, vertex_src);
            gl.compile_shader(vertex_shader);
            
            if !gl.get_shader_compile_status(vertex_shader) {
                let log = gl.get_shader_info_log(vertex_shader);
                gl.delete_shader(vertex_shader);
                return Err(format!("Vertex shader compilation failed: {}", log));
            }
            
            // Compile fragment shader
            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER)
                .map_err(|e| format!("Failed to create fragment shader: {}", e))?;
            
            gl.shader_source(fragment_shader, fragment_src);
            gl.compile_shader(fragment_shader);
            
            if !gl.get_shader_compile_status(fragment_shader) {
                let log = gl.get_shader_info_log(fragment_shader);
                gl.delete_shader(vertex_shader);
                gl.delete_shader(fragment_shader);
                return Err(format!("Fragment shader compilation failed: {}", log));
            }
            
            // Link program
            let program = gl.create_program()
                .map_err(|e| format!("Failed to create program: {}", e))?;
            
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            
            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);
                gl.delete_shader(vertex_shader);
                gl.delete_shader(fragment_shader);
                gl.delete_program(program);
                return Err(format!("Program linking failed: {}", log));
            }
            
            // Clean up shaders (they're linked into the program now)
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            info!("✅ Shader program compiled and linked successfully");
            
            Ok(Self { program })
        }
    }
    
    /// Use this shader program
    pub fn use_program(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
    
    /// Get uniform location
    pub fn get_uniform_location(&self, gl: &glow::Context, name: &str) -> Option<glow::UniformLocation> {
        unsafe {
            gl.get_uniform_location(self.program, name)
        }
    }
    
    /// Get attribute location
    pub fn get_attrib_location(&self, gl: &glow::Context, name: &str) -> Option<u32> {
        unsafe {
            gl.get_attrib_location(self.program, name)
        }
    }
}

/// Basic colored rectangle shader (OpenGL 3.3)
pub const BASIC_VERTEX_SHADER: &str = r#"#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;

out vec4 vColor;

void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);
    vColor = aColor;
}
"#;

pub const BASIC_FRAGMENT_SHADER: &str = r#"#version 330 core
in vec4 vColor;
out vec4 FragColor;

void main() {
    FragColor = vColor;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_sources_are_valid() {
        // Just verify the shader sources are non-empty and contain expected keywords
        assert!(BASIC_VERTEX_SHADER.contains("#version 330"));
        assert!(BASIC_VERTEX_SHADER.contains("gl_Position"));
        
        assert!(BASIC_FRAGMENT_SHADER.contains("#version 330"));
        assert!(BASIC_FRAGMENT_SHADER.contains("FragColor"));
    }
}
