use crate::checkpoint::Checkpoint;

use super::{cursor::TranscriberCursor, error::TranscriptionException, TranscriberTickResult};

pub struct TranscriptionIntent<ResultUnit, Error>(pub TranscriberTickResult<ResultUnit, Error>);

impl<ResultUnit, Error> TranscriptionIntent<ResultUnit, Error> {
    pub fn map<NewResultUnit, NewError>(self, map: impl Fn(TranscriberTickResult<ResultUnit, Error>) -> TranscriberTickResult<NewResultUnit, NewError>) -> TranscriptionIntent<NewResultUnit, NewError> {
        TranscriptionIntent(map(self.0))
    }

    pub fn alt(self, alt: impl FnOnce() -> TranscriptionIntent<ResultUnit, Error>) -> TranscriptionIntent<ResultUnit, Error> {
        if let Err(TranscriptionException::NotFound(_)) = self.0 {
            alt()
        } else {
            self
        }
    }

    pub fn tag(self, tag: String) -> TranscriberTickResult<ResultUnit, Error> {
        if let Err(TranscriptionException::NotFound(_)) = self.0 {
            Err(TranscriptionException::NotFound(tag))
        } else {
            self.0
        }
    }
}

// impl<ResultUnit, Error> From<TranscriberTickResult<ResultUnit, Error>> for TranscriptionIntent<ResultUnit, Error> {
//     fn from(result: TranscriberTickResult<ResultUnit, Error>) -> Self {
//         Self(result)
//     }
// }

impl<'a, Unit> TranscriberCursor<'a, Unit> {
    /// Try to transcribe, after the intent the cursor will be modified only if the transcription was successful
    pub fn intent<ResultUnit, Error>(&mut self, intent: impl Fn(&mut Self) -> TranscriberTickResult<ResultUnit, Error>) -> TranscriptionIntent<ResultUnit, Error> {
        // Save the status of the cursor before the intent
        let checkpoint = self.checkpoint();

        // Try to transcribe (intent)
        let result = intent(self);

        // If error, rollback the cursor
        if result.is_err() {
            self.rollback(checkpoint);
        }

        TranscriptionIntent(result)
    }
}
