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

                    Event::KeyDown {
                        keycode: Some(Keycode::F11),
                        ..
                    } => self.toggle_full_screen_state(),
                    Event::Window {
                        win_event: sdl2::event::WindowEvent::Resized(width, height),
                        ..
                    } => self.on_window_size_changed(core::Size::new(width, height)),
                    Event::KeyDown {
                        keycode: Some(Keycode::E),
                        // keymod: Mod::LCTRLMOD,
                        ..
                    } => self.export(),
                    Event::KeyDown {
                        keycode: Some(Keycode::C),
                        // keymod: Mod::LCTRLMOD,
                        ..
                    } => self.check_gl_error(),
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
    pub fn new(width: i32, height: i32) -> Result<Self, String> {
        Window::new_window_with_title_function(
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
}

impl Window {
    fn check_gl_error(&self) {
        util::print_debug!("gl_error: {:?}", self.graphic.check_gl_error())
    }
    fn export(&self) {
        util::print_info!("export trigered");
        self.graphic.export_shape_cache();
    }
    fn new_window_with_title_function(
        title_function: TitleCallback,
        width: i32,
        height: i32,
    ) -> Result<Self, String> {
        let sdl_context = util::expect!(sdl2::init());
        let video_subsystem = util::expect!(sdl_context.video());
        set_video_subsystem_attribute(&video_subsystem);
        // let parameter = if cfg!(target_arch = "aarch64") {
        //     WindowParameter::new(crate::VideoProfile::GLES, core::Version::new(2u8, 0u8, 0u8))
        // } else {
        //     WindowParameter::new(crate::VideoProfile::Core, core::Version::new(4u8, 0u8, 0u8))
        // };
        let parameter =
            WindowParameter::new(crate::VideoProfile::GLES, core::Version::new(2u8, 0u8, 0u8));

        set_gl_version(&video_subsystem, &parameter);
        let mut sdl_window = util::expect!(video_subsystem
            .window(&title_function(&parameter), width as _, height as _)
            .opengl()
            // .input_grabbed()
            .allow_highdpi()
            .build());

        let window_icon = <sdl2::surface::Surface as sdl2::image::LoadSurface>::from_file(
            "resource/image/icon/icon96.png",
        )?;
        sdl_window.set_icon(window_icon);
        sdl_window.set_resizable(false);
        if cfg!(target_arch = "aarch64") {
            sdl_context.mouse().show_cursor(false);
            sdl_window
                .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                .unwrap();
        }
        // let _ = sdl_window.set_fullscreen(sdl2::video::FullscreenType::True);

        // Unlike the other example above, nobody created a context for your window,
        // so you need to create one.
        let _gl_context = util::expect!(sdl_window.gl_create_context());

        let renderer = opengl::Renderer::new(|name| get_proc_address(&video_subsystem, name));
        let graphic = graphic::Graphic::new(Box::new(renderer));

        core::Graphic::init(&graphic, core::Size::new(width, height));

        let window = Window {
            parameter,
            sdl_context,
            video_subsystem,
            sdl_window,
            _gl_context,
            fps_counter: RefCell::new(FpsCounter::new(std::time::Duration::from_secs(2))),
            fps_limiter: None,
            title_function,
            background_color: core::Color::MagicDeepGray,
            graphic: Box::new(graphic),
        };
        Ok(window)
    }
    fn toggle_full_screen_state(&mut self) {
        match self.sdl_window.fullscreen_state() {
            sdl2::video::FullscreenType::Off => {
                self.sdl_window
                    .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                    .unwrap();
            }
            sdl2::video::FullscreenType::True | sdl2::video::FullscreenType::Desktop => {
                self.sdl_window
                    .set_fullscreen(sdl2::video::FullscreenType::Off)
                    .unwrap();
            }
        }
    }
    fn on_window_size_changed(&self, size: core::Size<i32>) {
        self.graphic.set_rendering_size(size);
        println!("on_size_changed: size: {:?}", size);
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

fn set_video_subsystem_attribute(video_subsystem: &VideoSubsystem) {
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
