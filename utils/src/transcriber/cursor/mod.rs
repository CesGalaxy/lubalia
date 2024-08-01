pub mod navigation;

#[cfg(test)]
mod tests;

use crate::checkpoint::Checkpoint;

/// A set of tools for moving through a vector of units with a cursor.
#[derive(Debug, Clone)]
pub struct TranscriberCursor<'a, Unit> {
    /// The position of the cursor, specifies the position of the current unit (starting at 0)
    pub pos: usize,

    // The source where the cursor moves through and gets the units from
    pub source: &'a Vec<Unit>,
}

impl <'a, Unit> Checkpoint<usize> for TranscriberCursor<'a, Unit> {
    /// Save the current cursor position
    fn checkpoint(&self) -> usize {
        self.pos
    }

    /// Rollback the cursor to the saved position
    fn rollback(&mut self, save: usize) {
        self.pos = save;
    }
}