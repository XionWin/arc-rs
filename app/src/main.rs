extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut window = arc_gl::window::Window::new("Arc | OpenGL", 800, 480);
    window.set_vsync(true);

    let context = window.get_render_context();
    let mut event_pump = context.create_event_pump();
    'running: loop {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
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
