extern crate gl;

const MIDNIGHT_BLUE: (f32, f32, f32, f32) = (25f32 / 255f32, 25f32 / 255f32, 112f32 / 255f32, 1f32);

fn main() {
    let mut window = platform::Window::new("Arc | OpenGL", 800, 480);
    window.set_vsync(true);

    window.run(
        |video_subsystem| {
            gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        },
        || unsafe {
            gl::ClearColor(
                MIDNIGHT_BLUE.0,
                MIDNIGHT_BLUE.1,
                MIDNIGHT_BLUE.2,
                MIDNIGHT_BLUE.3,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        },
    );
}
