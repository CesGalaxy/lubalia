pub struct Scope {
    pub indent: usize,
}

impl Scope {
    pub fn new(indent: usize) -> Self {
        Self { indent }
    }
}