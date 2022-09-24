/**
Now create the same 2 triangles using two different VAOs and VBOs for their data.
*/

use std::ffi::{c_void, CString};
use crate::utils::load_shader;
use crate::utils::validate_shader_program_linkage;
use std::intrinsics::sinf32;
use std::time::{SystemTime, UNIX_EPOCH};

const FIRST_SHADER_VERTEX: &str = include_str!("../shaders/vertex/first_shader.vert");
const SHADER_WITH_UNIFORM_FRAGMENT: &str = include_str!("../shaders/fragment/first_uniform_shader.frag");


unsafe fn create_vertex_buffer_object(vertices: [f32; 9]) -> u32 {
    // Vertex Buffer Object (VBO) is the object that contains all the vertices which are
    // passed to the GPU. Moving memory from the CPU to the GPU is costly therefore we
    // should do it all at once.
    let mut vertex_buffer_object: u32 = 0;
    gl::GenBuffers(1, &mut vertex_buffer_object);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<f32>() as i32, std::ptr::null() as *const c_void,
    );
    gl::EnableVertexAttribArray(0);

    vertex_buffer_object
}


pub fn get_dynamic_color() -> f32 {
    let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    unsafe {
        let seed = ((current_timestamp % 10000) as f32) / 500.0 % 360.0;
        println!("{}", seed);
        (sinf32(seed as f32) + 1.0) / 2.0
    }
}


pub fn main(on_loop_start: impl Fn(), on_loop_end: impl Fn()) {
    let vertex_shader = load_shader(FIRST_SHADER_VERTEX, gl::VERTEX_SHADER);
    let fragment_shader = load_shader(SHADER_WITH_UNIFORM_FRAGMENT, gl::FRAGMENT_SHADER);

    unsafe {
        let mut vertex_array_object: u32 = 0;
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::BindVertexArray(vertex_array_object);

        create_vertex_buffer_object([
            -0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
            0.5, -0.5, 0.0
        ]);

        // a "program" is the product of linking all the relevant shaders together.
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        validate_shader_program_linkage(shader_program);

        let color_name = CString::new("customColor").unwrap();
        let customColorLocation = gl::GetUniformLocation(shader_program, color_name.as_ptr());
        gl::UseProgram(shader_program);
        gl::Uniform4f(customColorLocation, 0.0, get_dynamic_color(), 0.0, 1.0);

        // Once the program is linked, we can delete the shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        loop {
            on_loop_start();

            // Set the background color to blue
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::Uniform4f(customColorLocation, 0.0, get_dynamic_color(), 0.0, 1.0);

            gl::BindVertexArray(vertex_array_object);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            on_loop_end();
        }
    }
}