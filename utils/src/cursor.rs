pub trait CursorNavigation<'a, Source, Unit> {
    /// Create a new cursor for a given source
    fn new(source: &'a Source) -> Self;

    /// Move the cursor to the next unit
    fn next(&mut self);

    /// Move the cursor to the previous unit
    fn back(&mut self);

    /// Move the cursor n units (negative moves backwards)
    fn move_by(&mut self, n: isize);

    /// Get the current unit (if cursor hasn't reached the end)
    fn peek(&self) -> Option<&'a Unit>;

    /// Get the next unit (if cursor hasn't reached the end)
    fn peek_next(&self) -> Option<&'a Unit>;

    /// Get the previous unit (if cursor hasn't reached the start)
    fn peek_prev(&self) -> Option<&'a Unit>;

    /// Peek at the n-th unit (negative moves backwards)
    fn peek_by(&self, n: isize) -> Option<&'a Unit>;

    // Peek at the n-th unit
    fn peek_at(&self, n: usize) -> Option<&'a Unit> {
        self.peek_by(n as isize)
    }

    /// Get the current unit and move the cursor forward to the next unit
    fn consume(&mut self) -> Option<&'a Unit> {
        self.next();
        self.peek_prev()
    }

    /// Check if the cursor has passed/reached the end of the source
    fn is_overflow(&self) -> bool;
}