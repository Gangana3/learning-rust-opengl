/**
Create two shader programs where the second program uses a different fragment shader
that outputs the color yello, draw both triangles again where one outputs the color yellow.
*/

use std::ffi::c_void;
use crate::utils::load_shader;
use crate::utils::validate_shader_program_linkage;

const FIRST_SHADER_VERTEX: &str = include_str!("../shaders/vertex/first_shader.vert");
const FIRST_SHADER_FRAGMENT: &str = include_str!("../shaders/fragment/first_shader.frag");
const YELLOW_SHADER_FRAGMENT: &str = include_str!("../shaders/fragment/yellow_shader.frag");


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


pub fn main(on_loop_start: impl Fn(), on_loop_end: impl Fn()) {
    let vertex_shader = load_shader(FIRST_SHADER_VERTEX, gl::VERTEX_SHADER);
    let fragment_shader = load_shader(FIRST_SHADER_FRAGMENT, gl::FRAGMENT_SHADER);
    let yellow_shader = load_shader(YELLOW_SHADER_FRAGMENT, gl::FRAGMENT_SHADER);

    unsafe {
        let mut vertex_array_object1: u32 = 0;
        gl::GenVertexArrays(1, &mut vertex_array_object1);
        gl::BindVertexArray(vertex_array_object1);

        create_vertex_buffer_object([
            -0.75, 0.0, 0.0,
            -0.25, 0.0, 0.0,
            -0.5, 0.25, 0.0
        ]);

        let mut vertex_array_object2: u32 = 0;
        gl::GenVertexArrays(1, &mut vertex_array_object2);
        gl::BindVertexArray(vertex_array_object2);

        create_vertex_buffer_object([
            0.25, 0.0, 0.0,
            0.75, 0.0, 0.0,
            0.5, 0.25, 0.0
        ]);

        // Create 2 programs this time
        let shader_program1 = gl::CreateProgram();
        let shader_program2 = gl::CreateProgram();

        // Link the first program
        gl::AttachShader(shader_program1, vertex_shader);
        gl::AttachShader(shader_program1, fragment_shader);
        gl::LinkProgram(shader_program1);
        validate_shader_program_linkage(shader_program1);

        // Link the second program (the one with the yellow shader).
        gl::AttachShader(shader_program2, vertex_shader);
        gl::AttachShader(shader_program2, yellow_shader);
        gl::LinkProgram(shader_program2);
        validate_shader_program_linkage(shader_program2);

        // Once the program is linked, we can delete the shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        loop {
            on_loop_start();

            // Set the background color to blue
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program1);
            gl::BindVertexArray(vertex_array_object1);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::UseProgram(shader_program2);
            gl::BindVertexArray(vertex_array_object2);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            on_loop_end();
        }
    }
}