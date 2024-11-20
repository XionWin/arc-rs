use core::Window;
use std::{cell::Cell, rc::Rc};

fn main() {
    util::print_hight_light!("====================[ARC DEMO]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(800, 480));
    window.set_vsync(true);

    let image: Option<Box<dyn core::Image>> = None;
    let rc_image = Rc::new(Cell::new(image));
    let rc_image2 = rc_image.clone();

    window.run(
        |window| {
            let _g = window.get_graphic();
            // _g.load_image_from_file("resource/image/icon96.png", core::ImageFilter::Linear)
            rc_image.replace(Some(_g.load_image_from_file("resource/image/icon96.png", core::ImageFilter::Linear)));
        },
        |window| {
            let _g = window.get_graphic();

            let r = rc_image2.take();
            match &r {
                Some(_v) => {},
                None => util::print_panic!("image is null")
            }
            rc_image2.set(r);
        }
    );
}
