use super::table::Table;

pub struct BufferPool {
    pub body: Vec<Table>,
}

impl BufferPool {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
}
