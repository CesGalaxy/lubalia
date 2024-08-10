#[derive(Debug, Clone)]
pub struct BlockMetadata {
    pub variables: usize,
}

impl Default for BlockMetadata {
    fn default() -> Self {
        BlockMetadata { variables: 0 }
    }
}