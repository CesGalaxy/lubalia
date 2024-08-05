use crate::checkpoint::Checkpoint;

use super::{cursor::TranscriberCursor, error::TranscriptionException, TranscriberTickResult};

pub struct TranscriptionIntent<ResultUnit, Error>(pub TranscriberTickResult<ResultUnit, Error>);

impl<ResultUnit, Error> TranscriptionIntent<ResultUnit, Error> {
    /// Map the result of the intent
    pub fn map<NewResultUnit, NewError>(self, map: impl Fn(TranscriberTickResult<ResultUnit, Error>) -> TranscriberTickResult<NewResultUnit, NewError>) -> TranscriptionIntent<NewResultUnit, NewError> {
        TranscriptionIntent(map(self.0))
    }

    /// In case of not founding any posibility of transcribing, try an alternative one
    pub fn alt(self, alt: impl FnOnce() -> TranscriptionIntent<ResultUnit, Error>) -> TranscriptionIntent<ResultUnit, Error> {
        if let Err(TranscriptionException::NotFound(_)) = self.0 {
            alt()
        } else {
            self
        }
    }

    /// Check the result of the intent and modify it if necessary
    pub fn check(self, check: impl FnOnce(&ResultUnit) -> Option<TranscriberTickResult<ResultUnit, Error>>) -> TranscriptionIntent<ResultUnit, Error> {
        if let Self(Ok(Some(unit))) = &self {
            check(unit).map(TranscriptionIntent).unwrap_or(self)
        } else {
            self
        }

    }

    /// In case nothing was found, tag the unit searched
    pub fn tag(self, tag: String) -> TranscriberTickResult<ResultUnit, Error> {
        if let Err(TranscriptionException::NotFound(_)) = self.0 {
            Err(TranscriptionException::NotFound(tag))
        } else {
            self.0
        }
    }
}

impl<'a, Unit> TranscriberCursor<'a, Unit> {
    /// Try to transcribe, after the intent the cursor will be modified only if the transcription was successful
    pub fn intent<ResultUnit, Error>(&mut self, mut intent: impl FnMut(&mut Self) -> TranscriberTickResult<ResultUnit, Error>) -> TranscriptionIntent<ResultUnit, Error> {
        // Save the status of the cursor before the intent
        let checkpoint = self.checkpoint();

        // Try to transcribe (intent)
        let result = intent(self);

        // If error, rollback the cursor
        if let Err(TranscriptionException::NotFound(_)) = &result {
            self.rollback(checkpoint);
        }

        TranscriptionIntent(result)
    }
}
