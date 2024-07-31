use crate::checkpoint::Checkpoint;

use super::{cursor::TranscriberCursor, TranscriberTickResult};

impl<'a, Unit> TranscriberCursor<'a, Unit> {
    /// Try to transcribe, after the intent the cursor will be modified only if the transcription was successful
    pub fn intent<ResultUnit, Error>(&mut self, intent: impl Fn(&mut Self) -> TranscriberTickResult<ResultUnit, Error>) -> TranscriberTickResult<ResultUnit, Error> {
        // Save the status of the cursor before the intent
        let checkpoint = self.checkpoint();

        // Try to transcribe (intent)
        let result = intent(self);

        // If error, rollback the cursor
        if result.is_err() {
            self.rollback(checkpoint);
        }

        result
    }
}