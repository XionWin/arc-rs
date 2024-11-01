

fn main() {
    let mut window = platform::Window::new("Arc | OpenGL", 800, 480);
    window.set_vsync(true);
  
    window.run(
        |video_subsystem| {
            // let loader = platform::loader::Loader::new("libGLESv2.so.2");
            gl::load_with(|name| {
                // loader.get_proc_address(name)
                video_subsystem.gl_get_proc_address(name) as *const _
            });
        },

        || {},
    );
}
