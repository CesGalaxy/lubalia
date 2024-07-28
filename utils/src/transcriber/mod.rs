pub mod cursor;
pub mod error;
pub mod result;

use cursor::TranscriberCursor;
use error::TranscriberError;
use result::{Transcription, TranscriptionResult};

pub type TranscriberTick<SourceUnit, ResultUnit, Error> = fn(&mut TranscriberCursor<SourceUnit>, &SourceUnit) -> TranscriberTickResult<ResultUnit, Error>;
pub type TranscriberTickResult<ResultUnit, Error> = Result<Option<ResultUnit>, Error>;

/// Transcribe a vec of iUnits into a vec of oUnits.
/// Create an iteration over the iUnits for transcribing them to oUnits
pub fn transcriber<SourceUnit: Clone, ResultUnit: std::fmt::Debug, Error: std::fmt::Display>(
    source: Vec<SourceUnit>,
    tick: TranscriberTick<SourceUnit, ResultUnit, Error>,
) -> TranscriptionResult<SourceUnit, ResultUnit, Error> {
    // Create an empty transcription result and a cursor for navigation through the source
    let mut transcription = Transcription::new(source.clone());
    let mut cursor = TranscriberCursor::new(&source);

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
                transcription.push(unit, Some(tick_initial_position), Some(current_position))
            },
            // If the tick fails, the transcription can't continue and the error is returned with additional information
            Err(error) => return Err(TranscriberError {
                tick_initial_position,
                tick_buffer: source[tick_initial_position..cursor.pos].to_vec(),
                cursor_position: cursor.pos,
                transcription_buffer: transcription,
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

    // Mark the transcription as completed
    transcription.completed = true;

    Ok(transcription)
}

#[cfg(test)]
mod tests;