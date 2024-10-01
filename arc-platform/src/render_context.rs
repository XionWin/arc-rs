use sdl2::EventPump;

pub struct RenderContext {
    pub(crate) sdl_context: sdl2::Sdl,
    pub(crate) video_subsystem: sdl2::VideoSubsystem,
    pub(crate) raw_window: sdl2::video::Window,
    pub(crate) _gl_content: sdl2::video::GLContext,
}

impl RenderContext {
    pub fn create_event_pump(&mut self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }
    pub fn swap_window(&self) {
        self.raw_window.gl_swap_window();
    }
}
