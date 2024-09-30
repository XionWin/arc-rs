use crate::render_context::RenderContext;

pub struct Window {
    context: RenderContext,
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
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        Window {
            context: RenderContext {
                sdl_context,
                video_subsystem,
                raw_window,
                _gl_content,
            },
        }
    }

    pub fn set_vsync(&self, is_open: bool) {
        let vsync_result = sdl2::VideoSubsystem::gl_set_swap_interval(
            &self.context.video_subsystem,
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

    pub fn get_render_context(&mut self) -> &mut RenderContext {
        &mut self.context
    }
}
