use crate::cursor::CursorNavigation;

use super::TranscriberCursor;

impl<'a, Unit> CursorNavigation<'a, Vec<Unit>, Unit> for TranscriberCursor<'a, Unit> {
    /// Create a new cursor for the given vec of units
    /// The cursor starts at the first unit (position 0)
    fn new(source: &'a Vec<Unit>) -> Self {
        Self { pos: 0, source }
    }

    /// Move forward the cursor
    fn next(&mut self) {
        self.pos += 1;
    }

    /// Move backward the cursor
    fn back(&mut self) {
        self.pos -= 1;
    }

    /// Get the unit at the cursor position
    fn peek(&self) -> Option<&'a Unit> {
        self.source.get(self.pos)
    }

    // Get the unit at the next cursor position
    fn peek_next(&self) -> Option<&'a Unit> {
        self.source.get(self.pos + 1)
    }

    /// Get the unit at the previous cursor position
    fn peek_prev(&self) -> Option<&'a Unit> {
        self.source.get(self.pos - 1)
    }

    /// Check if the cursor is outside the source (cursor position >= source length)
    fn is_overflow(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// Move the cursor n units (negative moves backwards)
    fn move_by(&mut self, n: isize) {
        if n < 0 {
            self.pos = self.pos.saturating_sub(n.abs() as usize);
        } else {
            self.pos = (self.pos as isize + n) as usize;
        }
    }
}