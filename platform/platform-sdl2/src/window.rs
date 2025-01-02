use std::{borrow::Borrow, cell::RefCell};

use crate::{fps_counter::FpsCounter, fps_limiter::FpsLimiter, WindowParameter};
use sdl2::{event::Event, keyboard::Keycode, VideoSubsystem};

type TitleCallback = fn(&WindowParameter) -> String;
pub struct Window {
    pub parameter: WindowParameter,
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) video_subsystem: sdl2::VideoSubsystem,
    pub(crate) sdl_window: sdl2::video::Window,
    pub(crate) _gl_context: sdl2::video::GLContext,
    pub(crate) fps_counter: RefCell<FpsCounter>,
    pub(crate) fps_limiter: Option<FpsLimiter>,
    pub title_function: TitleCallback,
    pub background_color: core::Color,
    graphic: Box<dyn core::Graphic>,
}

impl core::Window for Window {
    fn get_graphic(&self) -> &dyn core::Graphic {
        self.graphic.borrow()
    }

    fn run<T1, T2>(&mut self, on_load: T1, on_render: T2)
    where
        T1: Fn(&Self),
        T2: Fn(&Self),
    {
        self.init();
        on_load(self);
        let mut event_pump = util::expect!(self.sdl_context.event_pump());
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            self.begin_render();
            on_render(self);
            self.render();
            self.swap_buffers();
            match &mut self.fps_limiter {
                Some(fps_limiter) => fps_limiter.delay(),
                None => {}
            };
        }
    }

    fn set_vsync(&mut self, is_vsync: bool) {
        let vsync_result = sdl2::VideoSubsystem::gl_set_swap_interval(
            &self.video_subsystem,
            if is_vsync {
                sdl2::video::SwapInterval::VSync
            } else {
                sdl2::video::SwapInterval::Immediate
            },
        );
        self.fps_limiter = if vsync_result.is_err() {
            util::print_debug!("set vsync error, try use fps_limiter@60fps");
            Some(FpsLimiter::new(60))
        } else {
            util::print_debug!("set vsync successfully");
            None
        }
    }

    fn init(&self) {
        set_multisample(&self.video_subsystem);
        self.graphic.init();
    }

    fn begin_render(&self) {
        self.get_graphic().begin_render();
        self.fps_counter.borrow_mut().update(|fps| {
            util::print_debug!("fps: {fps:.0}");
        });
        self.get_graphic().clear_color(self.background_color);
        self.get_graphic().clear();
    }

    fn render(&self) {
        self.get_graphic().render();
    }

    fn swap_buffers(&self) {
        self.sdl_window.gl_swap_window();
    }
}

impl Window {
    pub fn new(width: u16, height: u16) -> Result<Self, String> {
        Window::new_with_title_function(
            |parameter| {
                String::from(format!(
                    "Arc | {:?} {}.{}",
                    parameter.profile, parameter.version.major, parameter.version.minor
                ))
            },
            width,
            height,
        )
    }

    pub fn new_with_title_function(
        title_function: TitleCallback,
        width: u16,
        height: u16,
    ) -> Result<Self, String> {
        let sdl_context = util::expect!(sdl2::init());
        let video_subsystem = util::expect!(sdl_context.video());
        let parameter = if cfg!(target_os = "macos") {
            WindowParameter::new(crate::VideoProfile::Core, core::Version::new(4u8, 0u8, 0u8))
        } else {
            WindowParameter::new(crate::VideoProfile::GLES, core::Version::new(2u8, 0u8, 0u8))
        };
        set_gl_version(&video_subsystem, &parameter);
        let mut sdl_window = util::expect!(video_subsystem
            .window(&title_function(&parameter), width.into(), height.into())
            .opengl()
            .build());

        let window_icon = <sdl2::surface::Surface as sdl2::image::LoadSurface>::from_file(
            "resource/image/icon/icon96.png",
        )?;
        sdl_window.set_icon(window_icon);
        sdl_window.set_resizable(false);
        // sdl_context.mouse().show_cursor(false);
        // let _ = sdl_window.set_fullscreen(sdl2::video::FullscreenType::True);

        // Unlike the other example above, nobody created a context for your window,
        // so you need to create one.
        let _gl_context = util::expect!(sdl_window.gl_create_context());

        let renderer = opengl::Renderer::new(core::Size::new(width as _, height as _), |name| {
            get_proc_address(&video_subsystem, name)
        });
        let graphic = graphic::Graphic::new(Box::new(renderer));
        Ok(Window {
            parameter,
            sdl_context,
            video_subsystem,
            sdl_window,
            _gl_context,
            fps_counter: RefCell::new(FpsCounter::new(std::time::Duration::from_secs(2))),
            fps_limiter: None,
            title_function,
            background_color: core::Color::MidnightBlue,
            graphic: Box::new(graphic),
        })
    }
}

pub fn get_proc_address(video_subsystem: &VideoSubsystem, name: &str) -> *const std::ffi::c_void {
    video_subsystem.gl_get_proc_address(name) as _
}

fn set_gl_version(video_subsystem: &VideoSubsystem, parameter: &WindowParameter) {
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(parameter.profile.into());
    gl_attr.set_context_version(parameter.version.major, parameter.version.minor);
}

pub fn set_multisample(video_subsystem: &VideoSubsystem) {
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_red_size(8);
    gl_attr.set_green_size(8);
    gl_attr.set_blue_size(8);
    gl_attr.set_alpha_size(8);

    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_accelerated_visual(true);
}
pub fn get_multisample(video_subsystem: &VideoSubsystem) {
    let gl_attr = video_subsystem.gl_attr();
    println!("multisample_samples: {:?}", gl_attr.multisample_samples());
}
