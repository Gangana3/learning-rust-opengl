/*
In this module I'm going to draw my first triangle (super exciting :) )
 */

pub fn draw_my_first_triangle() {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}