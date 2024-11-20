use std::{borrow::Borrow, rc::Rc};

use crate::{fps_counter::FpsCounter, fps_limiter::FpsLimiter, WindowParameter};
use sdl2::{event::Event, keyboard::Keycode, VideoSubsystem};

type TitleCallback = fn(&WindowParameter) -> String;
pub struct Window {
    pub parameter: WindowParameter,
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) video_subsystem: sdl2::VideoSubsystem,
    pub(crate) sdl_window: sdl2::video::Window,
    pub(crate) _gl_context: sdl2::video::GLContext,
    pub(crate) fps_counter: FpsCounter,
    pub(crate) fps_limiter: Option<FpsLimiter>,
    pub title_function: TitleCallback,
    graphic: Box<dyn core::Graphic>,
}

impl core::Window for Window {
    fn get_graphic(&self) -> &dyn core::Graphic {
        self.graphic.borrow()
    }

    fn run(&mut self, on_load: fn(&Self), on_render: fn(&Self)) {
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
            self.frame_init();
            on_render(self);
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
            util::print_debug!("set vsync error, try use fpslimiter at 60 fps");
            Some(FpsLimiter::new(60))
        } else {
            None
        }
    }

    fn init(&mut self) {}

    fn frame_init(&mut self) {
        self.fps_counter.update(|fps| {
            util::print_debug!("fps: {fps:.0}");
            // util::expect!(self.sdl_window.set_title(&format!(
            //     "{} fps: {:#.0}",
            //     &(self.title_function)("Core", &self.version),
            //     fps
            // )))
        });
        self.get_graphic().clear_color(core::Color::Black);
        self.get_graphic().clear();
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
        opengl::load_with(|name| get_proc_address(&video_subsystem, name));

        let window_icon = <sdl2::surface::Surface as sdl2::image::LoadSurface>::from_file(
            "resource/image/icon96.png",
        )?;
        sdl_window.set_icon(window_icon);
        sdl_window.set_resizable(false);

        // Unlike the other example above, nobody created a context for your window,
        // so you need to create one.
        let _gl_context = util::expect!(sdl_window.gl_create_context());
        let renderer = opengl::GLRenderer::new();
        let graphic = arc::Graphic::new(Rc::new(renderer));

        Ok(Window {
            parameter,
            sdl_context,
            video_subsystem,
            sdl_window,
            _gl_context,
            fps_counter: FpsCounter::new(std::time::Duration::from_secs(2)),
            fps_limiter: None,
            title_function,
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
