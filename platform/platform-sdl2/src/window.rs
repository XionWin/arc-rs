use opengl::gl;
use sdl2::{event::Event, keyboard::Keycode, VideoSubsystem};
use util::print_debug;

use crate::{fps_counter::FpsCounter, fps_limiter::FpsLimiter, WindowParameter};

const MIDNIGHT_BLUE: (f32, f32, f32, f32) = (25f32 / 255f32, 25f32 / 255f32, 112f32 / 255f32, 1f32);

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
}

impl arc::window::Window for Window {
    fn gl_get_proc_address(&self, procname: &str) -> *const std::ffi::c_void {
        self.video_subsystem.gl_get_proc_address(procname) as _
    }
}

impl Window {
    pub fn new(title_function: TitleCallback, width: u16, height: u16) -> Result<Self, String> {
        let sdl_context = util::expect!(sdl2::init());
        let video_subsystem = util::expect!(sdl_context.video());
        let parameter = 
        if cfg!(target_os = "macos") {
            WindowParameter::new(crate::VideoProfile::Core, core::Version::new(4u8, 0u8, 0u8))
        } else {
            WindowParameter::new(crate::VideoProfile::GLES, core::Version::new(2u8, 0u8, 0u8))
        };
        set_gl_version(&video_subsystem, &parameter);

        let mut sdl_window = util::expect!(video_subsystem
            .window(
                &title_function(&parameter),
                width.into(),
                height.into()
            )
            .opengl()
            .build());

        let window_icon = <sdl2::surface::Surface as sdl2::image::LoadSurface>::from_file(
            "resource/image/icon96.png",
        )?;
        sdl_window.set_icon(window_icon);
        sdl_window.set_resizable(false);

        // Unlike the other example above, nobody created a context for your window,
        // so you need to create one.
        let _gl_context = util::expect!(sdl_window.gl_create_context());

        Ok(Window {
            parameter,
            sdl_context,
            video_subsystem,
            sdl_window,
            _gl_context,
            fps_counter: FpsCounter::new(100),
            fps_limiter: None,
            title_function,
        })
    }

    pub fn set_vsync(&mut self, switch: bool) {
        let vsync_result = sdl2::VideoSubsystem::gl_set_swap_interval(
            &self.video_subsystem,
            if switch {
                sdl2::video::SwapInterval::VSync
            } else {
                sdl2::video::SwapInterval::Immediate
            },
        );
        self.fps_limiter = if vsync_result.is_err() {
            print_debug!("set vsync error, try use fpslimiter at 60 fps");
            Some(FpsLimiter::new(60))
        } else {
            None
        }
    }

    pub fn run<TLOAD, TRENDER>(&mut self, on_load: TLOAD, on_render: TRENDER)
    where
        TLOAD: Fn(&Window),
        TRENDER: Fn(),
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
                    _ => {}
                }
            }
            self.frame_init();
            on_render();
            self.swap_window();
            match &mut self.fps_limiter {
                Some(fps_limiter) => fps_limiter.delay(),
                None => {}
            };
        }
    }

    fn frame_init(&mut self) {
        self.fps_counter.update(|fps| {
            util::print_debug!("fps: {fps:.0}");
            // util::expect!(self.sdl_window.set_title(&format!(
            //     "{} fps: {:#.0}",
            //     &(self.title_function)("Core", &self.version),
            //     fps
            // )))
        });
        let (r, g, b, a) = MIDNIGHT_BLUE;
        gl::clear_color(r, g, b, a);
        gl::clear(
            opengl::ClearBufferMask::COLOR_BUFFER_BIT | opengl::ClearBufferMask::DEPTH_BUFFER_BIT,
        );
    }

    fn swap_window(&self) {
        self.sdl_window.gl_swap_window();
    }
}

fn set_gl_version(video_subsystem: &VideoSubsystem, parameter: &WindowParameter) {
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(parameter.profile.into());
    gl_attr.set_context_version(parameter.version.major, parameter.version.minor);
}
