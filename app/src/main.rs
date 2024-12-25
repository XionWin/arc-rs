use core::{Image, Style, Window};
use std::{
    borrow::Borrow,
    cell::RefCell,
    fs::DirBuilder,
    path,
    rc::{self, Rc},
};

fn main() {
    util::print_hight_light!("====================[Arc Demo]====================");
    let mut window = util::expect!(platform_sdl2::Window::new(800, 480));
    window.set_vsync(true);

    let image: RefCell<Option<Box<dyn core::Image>>> = RefCell::new(None);

    window.run(
        |window| {
            let g = window.get_graphic();
            image.replace(Some(g.load_image_from_file(
                "resource/image/icon/icon96.png",
                core::ImageFilter::Linear,
            )));

            let mut x = 0i32;
            let mut y = 0i32;
            let mut max_y = 0i32;
            let zoom_factor = 4i32;

            let paths = std::fs::read_dir(
                "/home/win/Downloads/Sunnyside_World_ASSET_PACK_V2.1/Sunnyside_World_Assets/UI/",
            )
            .unwrap()
            .filter(|item| item.as_ref().unwrap().file_type().unwrap().is_file())
            .map(|item| String::from(item.as_ref().unwrap().path().to_str().unwrap()))
            .collect::<Vec<String>>();

            println!("{:?}", paths);

            for path in &paths {
                let img: Rc<dyn Image> = g
                    .load_image_from_file(path, core::ImageFilter::Nearest)
                    .into();
                let size = core::Size::new(
                    img.get_size().width * zoom_factor,
                    img.get_size().height * zoom_factor,
                );

                if x + size.width > 800 {
                    x = 0i32;
                    y += max_y;
                }

                max_y = max_y.max(size.height);

                let rectangle = vector::Rectangle::new(
                    x,
                    y,
                    size.width,
                    size.height,
                    Style {
                        background: Box::new(core::ImageBackground::new(rc::Rc::new(
                            core::PaintImage::new(
                                img,
                                core::Rect::new(x, y, size.width, size.height),
                            ),
                        ))),
                        stroke: core::ColorBackground::new(core::Color::Red, core::Color::Blue),
                        stroke_width: Some(1i32),
                    },
                );
                g.draw_shape(Box::new(rectangle));

                x += size.width;
            }
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
