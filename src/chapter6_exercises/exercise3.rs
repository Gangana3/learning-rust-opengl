/**
Now create the same 2 triangles using two different VAOs and VBOs for their data.
*/
use std::ffi::{c_void, CString};
use std::intrinsics::{sinf32, size_of};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::shader_program::ShaderProgram;

const SHADER_WITH_COLOR_INPUT_VERTEX: &str = include_str!("../shaders/vertex/position_as_color.vert");
const POSITION_AS_COLOR_FRAGMENT: &str = include_str!("../shaders/fragment/shader_with_color_input.frag");


unsafe fn create_vertex_buffer_object(vertices: [f32; 9]) -> u32 {
    // This time the vertices also contain the color of each point.
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
        3 * std::mem::size_of::<f32>() as i32,
        0 as *const c_void,
    );

    gl::EnableVertexAttribArray(0);

    vertex_buffer_object
}

pub fn main(on_loop_start: impl Fn(), on_loop_end: impl Fn()) {
    unsafe {
        let mut vertex_array_object: u32 = 0;
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::BindVertexArray(vertex_array_object);

        create_vertex_buffer_object([
            -0.5, -0.5, 0.0,
            0.0, 0.5, 0.0,
            0.5, -0.5, 0.0
        ]);

        let shader_program = ShaderProgram::from_source(
            SHADER_WITH_COLOR_INPUT_VERTEX,
            POSITION_AS_COLOR_FRAGMENT,
        );
        shader_program.compile();

        loop {
            on_loop_start();

            // Set the background color to blue
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.apply();
            gl::BindVertexArray(vertex_array_object);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            on_loop_end();
        }
    }
}

