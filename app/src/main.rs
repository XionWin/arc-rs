fn main() {
    util::print_hight_light!("====================[ARC DEMO]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(
        |parameter| {
            String::from(format!(
                "Arc | {:?} {}.{}",
                parameter.profile, parameter.version.major, parameter.version.minor
            ))
        },
        800,
        480
    ));
    window.set_vsync(true);

    window.run(
        |video_subsystem| {
            opengl::gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        },
        || {},
    );
}
