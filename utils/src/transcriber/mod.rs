pub mod cursor;
pub mod error;
pub mod result;
pub mod intent;

#[cfg(test)]
mod tests;

use std::fmt;

use cursor::TranscriberCursor;
use error::{TranscriptionError, TranscriptionException};
use result::{IdentifiedTranscriptionUnit, Transcription, TranscriptionResult};

use crate::cursor::CursorNavigation;

pub type TranscriberTickResult<R, E> = Result<Option<R>, TranscriptionException<E>>;

/// Transcribe a vec of iUnits into a vec of oUnits.
/// Create an iteration over the iUnits for transcribing them to oUnits
pub fn transcriber<S: Clone, R, E: fmt::Display>(
    source: Vec<S>,
    mut tick: impl FnMut(&mut TranscriberCursor<S>, &S) -> TranscriberTickResult<R, E>
) -> TranscriptionResult<S, R, E> {
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
            Err(error) => return Err(TranscriptionError {
                tick_initial_position,
                tick_buffer: source[tick_initial_position..cursor.pos].to_vec(),
                cursor_position: cursor.pos,
                transcription_buffer: result,
                error,
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
