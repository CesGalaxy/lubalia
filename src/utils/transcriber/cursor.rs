/// A set of tools for moving through a vector of units with a cursor.
#[derive(Debug, Clone)]
pub struct TranscriberCursor<'a, Unit> {
    /// The position of the cursor, specifies the position of the current unit (starting at 0)
    pub pos: usize,
    pub source: &'a Vec<Unit>,
}

impl<'a, Unit> TranscriberCursor<'a, Unit> {
    /// Create a new cursor for the given vec of units
    /// The cursor starts at the first unit (position 0)
    pub fn new(source: &'a Vec<Unit>) -> Self {
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
    pub fn peek(&self) -> Option<&Unit> {
        self.source.get(self.pos)
    }

    // Get the unit at the next cursor position
    pub fn peek_next(&self) -> Option<&Unit> {
        self.source.get(self.pos + 1)
    }

    /// Get the unit at the previous cursor position
    pub fn peek_prev(&self) -> Option<&Unit> {
        self.source.get(self.pos - 1)
    }

    /// Get the unit at the cursor position and move the cursor forward
    pub fn consume(&mut self) -> Option<&Unit> {
        self.next();
        self.peek_prev()
    }

    /// Check if the cursor is outside the source (cursor position >= source length)
    pub fn is_overflow(&self) -> bool {
        self.pos >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let source = vec![1, 2, 3, 4, 5];
        let mut cursor = TranscriberCursor::new(&source);

        assert_eq!(cursor.pos, 0, "cursor position should be 0");
        assert_eq!(cursor.peek(), Some(&1), "cursor peek should be 1");
        assert_eq!(cursor.consume(), Some(&1), "cursor consume should be 1");
        assert_eq!(cursor.pos, 1, "cursor position should be 1");
        assert_eq!(cursor.peek(), Some(&2), "cursor peek should be 2");
        assert_eq!(cursor.consume(), Some(&2), "cursor consume should be 2");
        assert_eq!(cursor.pos, 2, "cursor position should be 2");
        assert_eq!(cursor.peek(), Some(&3), "cursor peek should be 3");

        cursor.back();
        assert_eq!(cursor.pos, 1, "cursor position should be 1");
        assert_eq!(cursor.peek(), Some(&2), "cursor peek should be 2");

        assert!(!cursor.is_overflow(), "cursor should not be overflow");
        cursor.pos = 5;
        assert!(cursor.is_overflow(), "cursor should be overflow");
    }
}