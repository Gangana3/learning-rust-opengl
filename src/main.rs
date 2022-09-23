extern crate sdl2;

use sdl2::event::Event;
use std::process::exit;
use sdl2::video::Window;

fn start_event_loop(sdl_context: &sdl2::Sdl, window: &Window) {
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    exit(0);
                }
                _ => { println!("Unhandled event event.") }
            }
        }
    }
}


fn open_window(sdl_context: &sdl2::Sdl) -> Window {
    let video = sdl_context.video().unwrap();
    video.window("My first OpenGL App!", 800, 600).resizable().build().unwrap()
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let window = open_window(&sdl_context);
    start_event_loop(&sdl_context, &window);
}
