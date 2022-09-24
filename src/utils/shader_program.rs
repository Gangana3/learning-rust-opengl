/*

 */

use std::fs::File;
use std::io::Read;
use std::ffi::CString;

/// Shader program that is composed of vertex shader and fragment shader.
///
///
/// Note that "compile" method must be called before "apply".
/// # Example
///
/// ```rust
/// use opengl::utils::shader_program::ShaderProgram;
///
/// let shader_program = ShaderProgram::new("/path/to/vertex_shader.glsl", "/path/to/vertex_shader.glsl");
/// shader_program.compile();
/// shader_program.apply();
/// ```
pub struct ShaderProgram {
    vertex_shader_source: String,
    fragment_shader_source: String,
    pub id: u32
}


impl ShaderProgram {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .expect(&format!("Vertex shader file path was not found. {}", vertex_shader_path));
        let mut fragment_shader_file = File::open(fragment_shader_path)
            .expect(&format!("Fragment shader file path was not found. {}", fragment_shader_path));

        let mut vertex_shader_source: String = String::new();
        let mut fragment_shader_source: String = String::new();

        vertex_shader_file.read_to_string(&mut vertex_shader_source);
        fragment_shader_file.read_to_string(&mut fragment_shader_source);

        return Self {
            vertex_shader_source,
            fragment_shader_source,
            id: unsafe {gl::CreateProgram()}
        }
    }

    pub fn from_source(vertex_shader_source_code: &str, fragment_shader_source_code: &str) -> Self {
        return Self {
            vertex_shader_source: vertex_shader_source_code.to_owned(),
            fragment_shader_source: fragment_shader_source_code.to_owned(),
            id: unsafe {gl::CreateProgram()}
        }
    }

    fn validate_shader_compilation(shader: u32) {
        let mut success: i32 = 0;
        let info_log: [u8; 512] = [0; 512];

        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            gl::GetShaderInfoLog(shader, 512, std::ptr::null::<i32>() as *mut i32,
                                 info_log.as_ptr() as *mut i8);
        }

        if success == 0 {
            panic!("Shader compilation failed. {}", String::from_utf8(Vec::from(info_log)).unwrap());
        }
    }

    fn validate_shader_program_linkage(program: u32) {
        let mut success: i32 = 0;
        let info_log: [u8; 512] = [0; 512];

        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            gl::GetProgramInfoLog(program, 512, std::ptr::null::<i32>() as *mut i32,
                                  info_log.as_ptr() as *mut i8);
        }

        if success == 0 {
            panic!("Shader program linkage failed. {}",
                   String::from_utf8(Vec::from(info_log)).unwrap());
        }
    }

    /// Load a shader source code to the GPU, and return the handle to the loaded
    /// shader.
    ///
    /// # Arguments
    /// * `shader_source_code` - Shader's code in GLSL langauage
    /// * `shader_type` - Type of the shader, such as gl::VERTEX_SHADER, gl::FRAGMENT_SHADER, etc...
    pub fn load_shader(shader_source_code: &str, shader_type: u32) -> u32 {
        let shader: u32;
        let shader_source_code_c_string = CString::new(shader_source_code).unwrap();

        unsafe {
            shader = gl::CreateShader(shader_type);
            gl::ShaderSource(
                shader,
                1,
                &shader_source_code_c_string.as_ptr(),
                std::ptr::null()
            );
            gl::CompileShader(shader);
            Self::validate_shader_compilation(shader)
        }

        shader
    }

    /// Compile the vertex and the fragment shaders, and than link them together.
    /// Panic if one of the steps failed.
    pub fn compile(&self) {
        let vertex_shader = Self::load_shader(&self.vertex_shader_source, gl::VERTEX_SHADER);
        Self::validate_shader_compilation(vertex_shader);

        let fragment_shader = Self::load_shader(&self.fragment_shader_source, gl::FRAGMENT_SHADER);
        Self::validate_shader_compilation(fragment_shader);

        // Link the program
        unsafe {
            gl::AttachShader(self.id, vertex_shader);
            gl::AttachShader(self.id, fragment_shader);
            gl::LinkProgram(self.id);
        }

        Self::validate_shader_program_linkage(self.id);

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
    }

    pub fn apply(&self) {
        unsafe {gl::UseProgram(self.id)};
    }


    fn get_uniform_location(&self, uniform_name: &str) -> i32 {
        let uniform_name_c_string = CString::new(uniform_name).unwrap();
        unsafe {gl::GetUniformLocation(self.id, uniform_name_c_string.as_ptr())}
    }
    pub fn set_bool(&self, uniform_name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(uniform_name), value as i32);
        }
    }

    pub fn set_int(&self, uniform_name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(uniform_name), value);
        }
    }

    pub fn set_float(&self, uniform_name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(uniform_name), value);
        }
    }
}

