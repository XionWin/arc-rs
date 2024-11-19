use core::Window;

fn main() {
    util::print_hight_light!("====================[ARC DEMO]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(800, 480));
    window.set_vsync(true);

    window.run(
        |window| {
            let _g = window.get_graphic();
            
            let _i = _g.create_image(core::Size { width: 10, height: 10 }, core::ColorType::Rgba, core::ImageFilter::Linear);
            let _i = _g.load_image_from_file("resource/image/icon96.png", core::ImageFilter::Linear);
        },
        |window| {
            let _g = window.get_graphic();
        }
    );
}
