extern crate gl;
extern crate opengl;
extern crate sdl2;

use std::ffi::c_void;
use std::process::exit;
use sdl2::event::{Event, WindowEvent};
use sdl2::video::Window;
use opengl::chapter_6_exercises::exercise3;


const INITIAL_WINDOW_WIDTH: u32 = 800;
const INITIAL_WINDOW_HEIGHT: u32 = 600;

fn handle_events(sdl_context: &sdl2::Sdl) {
    let mut event_pump = sdl_context.event_pump().unwrap();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                exit(0);
            }
            Event::Window { win_event, .. } => {
                if let WindowEvent::Resized(width, height) = win_event {
                    println!("Resizing!!!");
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
            }
            _ => { println!("Unhandled event event.") }
        }
    }
}


fn open_window(sdl_context: &sdl2::Sdl) -> Window {
    let video = sdl_context.video().unwrap();
    video.window("My first OpenGL App!", INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .resizable()
        .opengl()
        .build()
        .unwrap()
}

/// Prepare everything needed before we can start rendering some sh*t
fn initialize_gl<'a>(sdl_context: &sdl2::Sdl) {
    // Since OpenGL is not a library, it is a specification, it is the programmer's
    // responsibility to find the address of each function used from opengl. Instead of
    // loading each function before every usage, we could just load it automatically by
    // by providing the following closure to "gl" library.
    let load_callback = |s| {
        sdl_context.video().unwrap().gl_get_proc_address(s) as *const c_void
    };
    gl::load_with(load_callback);
    gl::Viewport::load_with(load_callback);
    unsafe { gl::Viewport(0, 0, INITIAL_WINDOW_WIDTH as i32, INITIAL_WINDOW_HEIGHT as i32); }
}

fn main() {
    // Initialize the SDL2 context
    let sdl_context = sdl2::init().unwrap();

    // Initialize & Open a new window
    let window = open_window(&sdl_context);

    // Initialize GL context
    let _gl_context = window.gl_create_context().unwrap();

    // Initialize everything needed for GL
    initialize_gl(&sdl_context);

    exercise3::main(
        || handle_events(&sdl_context),
        || window.gl_swap_window(),
    );
}
