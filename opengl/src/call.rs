use crate::CallType;

#[derive(Debug)]
pub struct Call {
    pub call_type: CallType,
    pub vertex_offset: usize,
    pub vertex_len: usize,
    pub uniform_offset: usize,
    pub texture: Option<Box<dyn graphic::Texture>>,
}

impl Call {
    pub fn new(
        call_type: CallType,
        vertex_offset: usize,
        vertex_len: usize,
        uniform_offset: usize,
        texture: Option<Box<dyn graphic::Texture>>,
    ) -> Self {
        Self {
            call_type,
            vertex_offset,
            vertex_len,
            uniform_offset,
            texture,
        }
    }
}
