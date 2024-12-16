#[derive(Debug)]
pub struct AttributeLocation {
    pub name: String,
    pub offset: usize,
    pub len: usize,
}

impl AttributeLocation {
    pub fn new(name: &str, offset: usize, len: usize) -> Self {
        Self {
            name: String::from(name),
            offset,
            len,
        }
    }
}
