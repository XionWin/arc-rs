use sdl2::{event::Event, keyboard::Keycode, VideoSubsystem};

pub struct Window {
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) video_subsystem: sdl2::VideoSubsystem,
    pub(crate) raw_window: sdl2::video::Window,
    pub(crate) _gl_content: sdl2::video::GLContext,
}

impl Window {
    pub fn new(title: &str, width: u16, height: u16) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 0);

        let raw_window = video_subsystem
            .window(title, width.into(), height.into())
            .opengl()
            .build()
            .unwrap();

        // Unlike the other example above, nobody created a context for your window, so you need to create one.
        let _gl_content = raw_window.gl_create_context().unwrap();

        Window {
            sdl_context,
            video_subsystem,
            raw_window,
            _gl_content,
        }
    }

    pub fn set_vsync(&self, is_open: bool) {
        let vsync_result = sdl2::VideoSubsystem::gl_set_swap_interval(
            &self.video_subsystem,
            if is_open {
                sdl2::video::SwapInterval::VSync
            } else {
                sdl2::video::SwapInterval::Immediate
            },
        );
        if vsync_result.is_err() {
            panic!("VSync set failed")
        }
    }

    pub fn swap_window(&self) {
        self.raw_window.gl_swap_window();
    }

    pub fn run<T1, T2>(&mut self, on_load: T1, on_render: T2)
    where
        T1: Fn(&VideoSubsystem),
        T2: Fn(),
    {
        on_load(&self.video_subsystem);
        let mut event_pump = self.sdl_context.event_pump().unwrap();
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
