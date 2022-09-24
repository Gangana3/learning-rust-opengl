use std::ffi::CString;

pub fn validate_shader_compilation(shader: u32) {
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
        validate_shader_compilation(shader)
    }

    shader
}

pub fn validate_shader_program_linkage(program: u32) {
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