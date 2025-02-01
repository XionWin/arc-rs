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
                true,
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
    let x = 0i32;
    let y = 0i32;

    // let paths = std::fs::read_dir("resource/image/png/")
    //     .unwrap()
    //     .filter(|item| {
    //         item.as_ref().unwrap().file_type().unwrap().is_file()
    //             && item
    //                 .as_ref()
    //                 .unwrap()
    //                 .file_name()
    //                 .to_str()
    //                 .unwrap()
    //                 .contains("png")
    //     })
    //     .map(|item| String::from(item.as_ref().unwrap().path().to_str().unwrap()))
    //     .collect::<Vec<String>>();
    // println!("{:?}", paths);

    {
        let img: Rc<dyn Image> = g
            .load_image_from_file("resource/image/png/1.png", core::ImageFilter::Nearest, true)
            .into();

        let rectangle = graphic::shape::Rectangle::new(
            x,
            y,
            rendering_size.get_width(),
            rendering_size.get_height(),
            Style::new(
                Box::new(core::ImageBackground::new(Rc::new(core::PaintImage::new(
                    img.clone(),
                    core::Rectangle::new(
                        x,
                        y,
                        rendering_size.get_width(),
                        rendering_size.get_height(),
                    ),
                )))),
                core::ColorBackground::new(core::Color::MoselleGreen, core::Color::MoselleGreen),
                Some(1i32),
            ),
        );
        g.add_shape(Box::new(rectangle));
    }

    let colors = [
        core::Color::MidnightBlue,
        core::Color::MoselleGreen,
        core::Color::MoselleGreen,
        core::Color::White,
    ];

    let x_count = 8;
    let y_count = 2;
    let item_size = 80;
    let gap_size = 4;
    let start_x =
        (g.get_rendering_size().get_width() - (item_size + gap_size) * ZOOM_FACTOR * x_count) / 2;
    let start_y =
        g.get_rendering_size().get_height() - (item_size + gap_size) * ZOOM_FACTOR * y_count;
    for row in 0..y_count {
        for column in 0..x_count {
            let rectangle = graphic::shape::RoundRectangle::new(
                start_x + column * (item_size + gap_size) * ZOOM_FACTOR,
                start_y + row * (item_size + gap_size) * ZOOM_FACTOR,
                item_size * ZOOM_FACTOR,
                item_size * ZOOM_FACTOR,
                32 * ZOOM_FACTOR,
                Style::new(
                    Box::new(core::ColorBackground::new(
                        colors[((row * 2 + column) % (colors.len() as i32)) as usize],
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
