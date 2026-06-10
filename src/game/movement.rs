#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Movement {
    pub column: usize,
}

impl Movement {
    pub fn new(column: usize) -> Self {
        Self { column }
    }
}
