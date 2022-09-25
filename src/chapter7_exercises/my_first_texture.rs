/**
In this module I'll be drawing my first texture.
*/

use std::ffi::{c_void, CString};
use std::intrinsics::{sinf32, size_of};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::shader_program::ShaderProgram;

const FIRST_TEXTURE_SHADER_VERTEX: &str = include_str!("../shaders/vertex/first_texture_shader.vert");
const FIRST_TEXTURE_SHADER_FRAGMENT: &str = include_str!("../shaders/fragment/first_texture_shader.frag");


unsafe fn create_vertex_buffer_object(vertices: [f32; 32]) -> u32 {
    // This time each vertex contains also: RGB color, Texture coordinates.
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


pub fn get_dynamic_color() -> f32 {
    let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    unsafe {
        let seed = ((current_timestamp % 10000) as f32) / 500.0 % 360.0;
        println!("{}", seed);
        (sinf32(seed as f32) + 1.0) / 2.0
    }
}


pub fn main(on_loop_start: impl Fn(), on_loop_end: impl Fn()) {
    unsafe {
        let mut vertex_array_object: u32 = 0;
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::BindVertexArray(vertex_array_object);

        create_vertex_buffer_object([
            //  Vertices   |     Colors    |  Tex coordinates
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
            0.5, 0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
            0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0
        ]);
    }
    let shader_program = ShaderProgram::from_source(
        FIRST_TEXTURE_SHADER_VERTEX,
        FIRST_TEXTURE_SHADER_FRAGMENT,
    );
    shader_program.compile();

    loop {
        on_loop_start();

        unsafe {
            // Set the background color to blue
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.apply();
            gl::BindVertexArray(vertex_array_object);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        on_loop_end();
    }
}
