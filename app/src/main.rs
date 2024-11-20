use core::Window;
use std::cell::RefCell;

fn main() {
    util::print_hight_light!("====================[ARC DEMO]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(800, 480));
    window.set_vsync(true);

    let image: Option<Box<dyn core::Image>> = None;
    let image_ref = RefCell::new(image);

    window.run(
        |window| {
            let _g = window.get_graphic();
            image_ref.replace(Some(_g.load_image_from_file(
                "resource/image/icon96.png",
                core::ImageFilter::Linear,
            )));
        },
        |window| {
            let _g = window.get_graphic();

            let image = image_ref.take();
            match &image {
                Some(_v) => {}
                None => util::print_panic!("image is null"),
            }
            image_ref.replace(image);
        },
    );
}
