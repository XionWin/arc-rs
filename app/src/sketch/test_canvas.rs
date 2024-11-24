pub struct TestCanvas {
    vertex: Vec<core::Vertex2>,
}

impl TestCanvas {
    pub fn new() -> Self {
        Self {
            vertex: vec![core::Vertex2::new(0, 0, 0u8, 0u8); 3],
        }
    }

    pub fn load(&self) {
        util::print_debug!("test_canvas load")
    }

    pub fn render(&self) {}
}
