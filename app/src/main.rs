use core::{Style, Window};
use std::{cell::RefCell, rc};

fn main() {
    util::print_hight_light!("====================[Arc Demo]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(800, 480));
    window.set_vsync(true);

    let image: RefCell<Option<Box<dyn core::Image>>> = RefCell::new(None);

    window.run(
        |window| {
            let g = window.get_graphic();
            image.replace(Some(g.load_image_from_file(
                "resource/image/icon96.png",
                core::ImageFilter::Linear,
            )));

            let img = g
                .load_image_from_file("resource/image/icon96.png", core::ImageFilter::Linear)
                .into();
            let rectangle = vector::Rectangle::new(
                100,
                100,
                96,
                96,
                Style {
                    background: Box::new(core::ImageBackground::new(rc::Rc::new(
                        core::PaintImage::new(img, core::Rect::new(100, 100, 96, 96)),
                    ))),
                    stroke: core::ColorBackground::new(core::Color::Red, core::Color::Blue),
                    stroke_width: Some(1i32),
                },
            );
            g.draw_shape(Box::new(rectangle));
        },
        |window| {
            let _g = window.get_graphic();

            match image.borrow().as_ref() {
                Some(_v) => {}
                None => util::print_panic!("image is null"),
            }
        },
    );
}
