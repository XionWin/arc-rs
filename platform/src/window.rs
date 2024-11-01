use opengl::gl;
use sdl2::{event::Event, keyboard::Keycode, VideoSubsystem};

use crate::{fps_counter::FpsCounter, fps_limiter::FpsLimiter};


const MIDNIGHT_BLUE: (f32, f32, f32, f32) = (25f32 / 255f32, 25f32 / 255f32, 112f32 / 255f32, 1f32);

pub struct Window {
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) video_subsystem: sdl2::VideoSubsystem,
    pub(crate) sdl_window: sdl2::video::Window,
    pub(crate) _gl_context: sdl2::video::GLContext,
    pub(crate) fps_limiter: Option<FpsLimiter>,
}

impl Window {
    pub fn new(title: &str, width: u16, height: u16) -> Self {
        let sdl_context = sdl2::init().expect("SDLContext create failed");
        let video_subsystem = sdl_context.video().expect("VideoSubsystem create failed");

        let gl_attr = video_subsystem.gl_attr();
        if cfg!(target_os = "macos") {
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(4, 0);
        } else {
            gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
            gl_attr.set_context_version(2, 0);
        }

        let sdl_window = video_subsystem
            .window(title, width.into(), height.into())
            .opengl()
            .build()
            .expect("Window create failed");

        // Unlike the other example above, nobody created a context for your window,
        // so you need to create one.
        let _gl_context = sdl_window
            .gl_create_context()
            .expect("GlContext create failed");

        Window {
            sdl_context,
            video_subsystem,
            sdl_window,
            _gl_context,
            fps_limiter: None,
        }
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
            Some(FpsLimiter::new(60))
        } else {
            None
        }
    }

    pub fn swap_window(&self) {
        self.sdl_window.gl_swap_window();
    }

    pub fn run<TLOAD, TRENDER>(&mut self, on_load: TLOAD, on_render: TRENDER)
    where
        TLOAD: Fn(&VideoSubsystem),
        TRENDER: Fn(),
    {
        let mut fps_counter = FpsCounter::new(100);

        on_load(&self.video_subsystem);
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .expect("Event pump get failed");
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
            fps_counter.update();
            self.refresh();
            on_render();
            self.swap_window();
            match &mut self.fps_limiter {
                Some(fps_limiter) => fps_limiter.delay(),
                None => {}
            };
        }
    }
    fn refresh(&self) {
        let (r, g, b, a) = MIDNIGHT_BLUE;
        gl::clear_color(r, g, b, a);
        gl::clear(opengl::ClearBufferMask::COLOR_BUFFER_BIT | opengl::ClearBufferMask::DEPTH_BUFFER_BIT);
    }
}
