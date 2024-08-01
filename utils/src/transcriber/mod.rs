pub mod cursor;
pub mod error;
pub mod result;
pub mod intent;

#[cfg(test)]
mod tests;

use std::fmt;

use cursor::TranscriberCursor;
use error::{TranscriberError, TranscriptionException};
use result::{IdentifiedTranscriptionUnit, Transcription, TranscriptionResult};

use crate::cursor::CursorNavigation;

pub type TranscriberTick<SourceUnit, ResultUnit, Error> = fn(&mut TranscriberCursor<SourceUnit>, &SourceUnit) -> TranscriberTickResult<ResultUnit, Error>;
pub type TranscriberTickResult<ResultUnit, Error> = Result<Option<ResultUnit>, TranscriptionException<Error>>;

/// Transcribe a vec of iUnits into a vec of oUnits.
/// Create an iteration over the iUnits for transcribing them to oUnits
pub fn transcriber<SourceUnit: Clone, ResultUnit: fmt::Debug, Error: fmt::Display>(
    source: Vec<SourceUnit>,
    tick: TranscriberTick<SourceUnit, ResultUnit, Error>,
) -> TranscriptionResult<SourceUnit, ResultUnit, Error> {
    let mut cursor = TranscriberCursor::new(&source);
    let mut result = Vec::new();

    // Iterate over the source units
    while let Some(tick_initial_unit) = cursor.peek().map(|tiu| tiu.clone()) {
        // The position of the cursor when the tick started
        let tick_initial_position = cursor.pos;

        // Execute the tick function
        let tick_result = tick(&mut cursor, &tick_initial_unit);

        match tick_result {
            // If the tick is successful, and it transcribed something,
            // the result is added to the transcription with additional information
            Ok(Some(unit)) => {
                let current_position = cursor.pos;
                result.push(IdentifiedTranscriptionUnit::new(unit, Some(tick_initial_position), Some(current_position)))
            },
            // If the tick fails, the transcription can't continue and the error is returned with additional information
            Err(error) => return Err(TranscriberError {
                tick_initial_position,
                tick_buffer: source[tick_initial_position..cursor.pos].to_vec(),
                cursor_position: cursor.pos,
                transcription_buffer: result,
                error: error,
            }),
            // The tick can return None if there's nothing to transcribe
            _ => {}
        }

        // If the cursor didn't move, move it forward one unit
        // This can be useful for ticks that are ignored (like comments, spaces, end of line, etc.)
        if tick_initial_position == cursor.pos {
            cursor.next();
        }
    }

    Ok(Transcription::new(result, source))
}
