use sdl2::{event::Event, keyboard::Keycode, VideoSubsystem};

pub struct Window {
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) video_subsystem: sdl2::VideoSubsystem,
    pub(crate) sdl_window: sdl2::video::Window,
    pub(crate) _gl_context: sdl2::video::GLContext,
}

impl Window {
    pub fn new(title: &str, width: u16, height: u16) -> Self {
        let sdl_context = sdl2::init().expect("SDLContext create failed");
        let video_subsystem = sdl_context.video().expect("VideoSubsystem create failed");

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 0);

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
        }
    }

    pub fn set_vsync(&self, switch: bool) -> Result<(), String> {
        sdl2::VideoSubsystem::gl_set_swap_interval(
            &self.video_subsystem,
            if switch {
                sdl2::video::SwapInterval::VSync
            } else {
                sdl2::video::SwapInterval::Immediate
            },
        )
    }

    pub fn swap_window(&self) {
        self.sdl_window.gl_swap_window();
    }

    pub fn run<TLOAD, TRENDER>(&mut self, on_load: TLOAD, on_render: TRENDER)
    where
        TLOAD: Fn(&VideoSubsystem),
        TRENDER: Fn(),
    {
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
            on_render();
            self.swap_window();
        }
    }
}
