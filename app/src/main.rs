use core::{Image, Style, Window};
use std::{cell::RefCell, rc::Rc};

const ZOOM_FACTOR: i32 = if cfg!(target_arch = "aarch64") {
    1i32
} else {
    2i32
};

fn main() {
    util::print_hight_light!("====================[Arc Demo]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(
        800 * ZOOM_FACTOR,
        480 * ZOOM_FACTOR
    ));
    window.set_vsync(true);

    let image: RefCell<Option<Box<dyn core::Image>>> = RefCell::new(None);

    window.run(
        |window| {
            let g = window.get_graphic();
            image.replace(Some(g.load_image_from_file(
                "resource/image/icon/icon96.png",
                core::ImageFilter::Linear,
            )));
            test(g);
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

fn test(g: &dyn core::Graphic) {
    let rendering_size = g.get_rendering_size();
    let mut x = 0i32;
    let mut y = 0i32;
    let mut max_y = 0i32;

    let paths = std::fs::read_dir("resource/image/png/")
        .unwrap()
        .filter(|item| {
            item.as_ref().unwrap().file_type().unwrap().is_file()
                && item
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .contains("png")
        })
        .map(|item| String::from(item.as_ref().unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();

    println!("{:?}", paths);

    for path in &paths {
        let img: Rc<dyn Image> = g
            .load_image_from_file(path, core::ImageFilter::Nearest)
            .into();
        let size = img.get_size().mul(ZOOM_FACTOR);
        if x + size.get_width() > rendering_size.get_width() {
            x = 0i32;
            y += max_y;
        }
        max_y = max_y.max(size.get_height());

        let rectangle = vector::Rectangle::new(
            x,
            y,
            size.get_width(),
            size.get_height(),
            Style::new(
                Box::new(core::ImageBackground::new(Rc::new(core::PaintImage::new(
                    img.clone(),
                    core::Rect::new(x, y, size.get_width(), size.get_height()),
                )))),
                core::ColorBackground::new(core::Color::MoselleGreen, core::Color::MoselleGreen),
                Some(1i32),
            ),
        );
        g.add_shape(Box::new(rectangle));
        x += size.get_width();
    }

    let colors = [
        core::Color::MidnightBlue,
        core::Color::MoselleGreen,
        core::Color::MoselleGreen,
        core::Color::White,
    ];
    for row in 0..2 {
        for column in 0..2 {
            let rectangle = vector::RoundRectangle::new(
                198 * ZOOM_FACTOR + column * (26 + 1) * ZOOM_FACTOR,
                122 * ZOOM_FACTOR + row * (26 + 1) * ZOOM_FACTOR,
                26 * ZOOM_FACTOR,
                26 * ZOOM_FACTOR,
                8 * ZOOM_FACTOR,
                Style::new(
                    Box::new(core::ColorBackground::new(
                        colors[(row * 2 + column) as usize],
                        core::Color::MoselleGreen,
                    )),
                    core::ColorBackground::new(
                        core::Color::MoselleGreen,
                        core::Color::MoselleGreen,
                    ),
                    Some(1i32),
                ),
            );
            g.add_shape(Box::new(rectangle));
        }
    }
}
