pub trait RenderingComponent: core::Graphic {
    fn get_renderer(&self) -> &dyn crate::Renderer;
}