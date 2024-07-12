pub struct TokenMetadata {
    pub location: (usize, usize),
}

impl TokenMetadata {
    pub fn new(pos: usize, length: usize) -> Self {
        Self {
            location: (pos, length)
        }
    }
}