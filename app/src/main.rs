use arc::Window;

fn main() {
    util::print_hight_light!("====================[ARC DEMO]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(800, 480));
    window.set_vsync(true);

    window.run(
        |window| {
            opengl::gl::load_with(|name| window.gl_get_proc_address(name) as *const _);
        },
        |window| {
            let _g = window.get_graphic();
        }
    );
}
