extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const MIDNIGHT_BLUE: (f32, f32, f32, f32) = (25f32 / 255f32, 25f32 / 255f32, 112f32 / 255f32, 1f32);

fn main() {
    let mut window = arc_gl::window::Window::new("Arc | OpenGL", 800, 480);
    window.set_vsync(true);

    let context = window.get_render_context();
    let mut event_pump = context.create_event_pump();
    'running: loop {
        unsafe {
            gl::ClearColor(
                MIDNIGHT_BLUE.0,
                MIDNIGHT_BLUE.1,
                MIDNIGHT_BLUE.2,
                MIDNIGHT_BLUE.3,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        context.swap_window();
    }
}
