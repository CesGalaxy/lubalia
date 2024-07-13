/// A set of tools for moving through a vector with a cursor.
pub struct TranscriberCursor<'a, SourceUnit> {
    pub pos: usize,
    pub source: &'a Vec<SourceUnit>,
}

impl<'a, SourceUnit> TranscriberCursor<'a, SourceUnit> {
    /// Create a new cursor for the given source
    pub fn new(source: &'a Vec<SourceUnit>) -> Self {
        Self { pos: 0, source }
    }

    /// Move forward the cursor
    pub fn next(&mut self) {
        self.pos += 1;
    }

    /// Move backward the cursor
    pub fn back(&mut self) {
        self.pos -= 1;
    }

    /// Get the unit at the cursor position
    pub fn peek(&self) -> Option<&SourceUnit> {
        self.source.get(self.pos)
    }

    /// Get the unit at the cursor position and move the cursor forward
    pub fn consume(&mut self) -> Option<&SourceUnit> {
        let unit = self.peek();
        self.next();
        unit
    }

    /// Check if the cursor is outside the source (cursor position >= source length)
    pub fn is_overflow(&self) -> bool {
        self.pos >= self.source.len()
    }
}