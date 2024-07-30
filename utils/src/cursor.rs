pub trait CursorNavigation<'a, Source, Unit> {
    /// Create a new cursor for a given source
    fn new(source: &'a Source) -> Self;

    /// Move the cursor to the next unit
    fn next(&mut self);

    /// Move the cursor to the previous unit
    fn back(&mut self);

    /// Get the current unit (if cursor hasn't reached the end)
    fn peek(&self) -> Option<&'a Unit>;

    /// Get the next unit (if cursor hasn't reached the end)
    fn peek_next(&self) -> Option<&'a Unit>;

    /// Get the previous unit (if cursor hasn't reached the start)
    fn peek_prev(&self) -> Option<&'a Unit>;

    /// Get the current unit and move the cursor forward to the next unit
    fn consume(&mut self) -> Option<&'a Unit> {
        self.next();
        self.peek_prev()
    }

    /// Check if the cursor has passed/reached the end of the source
    fn is_overflow(&self) -> bool;
}