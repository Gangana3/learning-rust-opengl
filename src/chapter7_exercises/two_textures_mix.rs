/**
In this module I'll be drawing my first texture.
*/

use std::ffi::{c_void, CString};
use std::intrinsics::{sinf32, size_of};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::shader_program::ShaderProgram;
use image::{ImageFormat, EncodableLayout};

const FIRST_TEXTURE_SHADER_VERTEX: &str = include_str!("../shaders/vertex/first_texture_shader.vert");
const FIRST_TEXTURE_SHADER_FRAGMENT: &str = include_str!("../shaders/fragment/mix_two_textures.frag");


unsafe fn create_vertex_buffer_object(vertices: &[f32]) -> u32 {
    // This time each vertex contains also: RGB color, Texture coordinates.
    let mut vertex_buffer_object: u32 = 0;
    gl::GenBuffers(1, &mut vertex_buffer_object);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * size_of::<f32>()) as isize,
        vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

    // 3D coordinates attribute
    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        8 * size_of::<f32>() as i32,
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(0);

    // Color attribute
    gl::VertexAttribPointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        8 * size_of::<f32>() as i32,
        (3 * size_of::<f32>()) as *const c_void,
    );
    gl::EnableVertexAttribArray(1);


    // Texture coordinates
    gl::VertexAttribPointer(
        2,
        2,
        gl::FLOAT,
        gl::FALSE,
        8 * size_of::<f32>() as i32,
        (6 * size_of::<f32>()) as *const c_void
    );
    gl::EnableVertexAttribArray(2);

    vertex_buffer_object
}

/// Create an EBO - a buffer of indices to the vertices specified in
/// the VBO
unsafe fn create_element_buffer_object(indices: &[u32]) -> u32 {
    let mut element_buffer_object: u32 = 0;
    gl::GenBuffers(1, &mut element_buffer_object);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * size_of::<u32>()) as isize,
        indices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    element_buffer_object
}

pub fn create_texture(texture_image_blob: &[u8]) -> u32 {
    let mut texture: u32 = 0;
    let image = image::load_from_memory(texture_image_blob).unwrap().flipv().to_rgb8();
    let image_pixels = image.as_bytes();

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Set the texture wrapping & filtering options
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // This line prevents the image from rendering without color.
        // By default the unpack alignment is 4, but an RGB image is working with multiplies of
        // 3 (R, G, B) and is tightly packed. So the alignment of 4 is wrong.
        // By doing the following we make openGL not to think that each image row is aligned
        // to 4 bytes as it does by default
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::TexImage2D(
            gl::TEXTURE_2D,                 // We are using 2D texture
            0,                               // Mipmap level to create texture for
            gl::RGB as i32,           // How the image blob is stored internally
            image.width() as i32,
            image.height() as i32,
            0,                              // Should always be 0 (legacy)
            gl::RGB,                        // Source image format.
            gl::UNSIGNED_BYTE,               // Our data is saved as u8, (== unsigned byte)
            image_pixels.as_ptr() as *const c_void,   // Image content
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    texture
}


pub fn main(on_loop_start: impl Fn(), on_loop_end: impl Fn(), i: i32) {
    unsafe {
        let mut vertex_array_object: u32 = 0;
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::BindVertexArray(vertex_array_object);

        create_vertex_buffer_object(&[
            // positions          // colors           // texture coords
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
            -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0  // top left
        ]);

        create_element_buffer_object(&[
            0, 1, 3,    // first triangle
            1, 2, 3     // second triangle
        ]);

        let shader_program = ShaderProgram::from_source(
            FIRST_TEXTURE_SHADER_VERTEX,
            FIRST_TEXTURE_SHADER_FRAGMENT,
        );
        shader_program.compile();

        let texture1 = create_texture(include_bytes!("../textures/awesomeface.png"));
        let texture2 = create_texture(include_bytes!("../textures/lava_texture.png"));


        gl::ActiveTexture(gl::TEXTURE0);    // Activate the first texture unit
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        gl::ActiveTexture(gl::TEXTURE1);    // Activate the second texture unit
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        gl::BindVertexArray(vertex_array_object);


        loop {
            on_loop_start();

            // Set the background color to blue
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.apply();
            shader_program.set_int("texture1", 0);
            shader_program.set_int("texture2", 1);

            gl::DrawElements(
                gl::TRIANGLES, 6,
                gl::UNSIGNED_INT,
                0 as *const c_void,
            );

            on_loop_end();
        }
    }
}

