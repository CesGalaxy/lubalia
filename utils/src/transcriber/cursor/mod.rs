pub mod navigation;

#[cfg(test)]
mod tests;

use crate::{checkpoint::Checkpoint, cursor::CursorNavigation, loop_through::LoopThrough};

use super::error::TranscriptionException;

/// A set of tools for moving through a vector of units with a cursor.
#[derive(Debug, Clone)]
pub struct TranscriberCursor<'a, Unit> {
    /// The position of the cursor, specifies the position of the current unit (starting at 0)
    pub pos: usize,

    // The source where the cursor moves through and gets the units from
    pub source: &'a Vec<Unit>,
}

impl<'a, Unit> Checkpoint<usize> for TranscriberCursor<'a, Unit> {
    /// Save the current cursor position
    fn checkpoint(&self) -> usize {
        self.pos
    }

    /// Rollback the cursor to the saved position
    fn rollback(&mut self, save: usize) {
        self.pos = save;
    }
}

impl<'a, Unit> TranscriberCursor<'a, Unit> {
    /// Checks if the cursor is at an expected token, if it is, it will be consumed and returned.
    pub fn expect<'b, Error>(&mut self, expectation: &'b Unit, error: Error) -> Result<&'b Unit, TranscriptionException<Error>> where Unit: PartialEq {
        if self.peek() == Some(expectation) {
            self.next();
            Ok(expectation)
        } else {
            Err(TranscriptionException::Error(error))
        }
    }

    /// Ignore all tokens which match the expectation
    pub fn ignore_all<'b>(&mut self, expectation: &'b Unit) -> usize where Unit: PartialEq {
        let mut count = 0;

        while self.peek() == Some(expectation) {
            self.next();
            count += 1;
        }

        count
    }

    pub fn ignore_loop<'b>(&mut self, condition: LoopThrough<'b, Unit>) -> usize where Unit: PartialEq {
        condition.over(self)
    }
}