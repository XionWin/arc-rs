pub trait F32ArrayValue {
    fn value(&self) -> Vec<f32>;
}

pub struct Vertex {
    pub x: f32,
}

impl F32ArrayValue for Vertex {
    fn value(&self) -> Vec<f32> {
        vec![0f32]
    }
    // add code here
}
