pub mod cursor;
pub mod error;
pub mod result;

use cursor::TranscriberCursor;
use error::TranscriberError;
use result::{Transcription, TranscriptionResult};

/// Transcribe a vec of iUnits into a vec of oUnits.
/// Create an iteration over the iUnits for transcribing them to oUnits
pub fn transcriber<SourceUnit: Clone, ResultUnit: std::fmt::Debug, Error: std::fmt::Display>(
    source: Vec<SourceUnit>,
    tick: impl Fn(&mut TranscriberCursor<SourceUnit>, &SourceUnit) -> Result<Option<ResultUnit>, Error>,
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
mod tests {
    use super::*;

    #[test]
    fn test_transcriber() {
        let source = vec![1, 2, 3, 4, 5];
        let result = transcriber::<u8, u8, &str>(source, |_, unit| {
            if *unit == 3 {
                Ok(Some(*unit))
            } else {
                Ok(None)
            }
        });

        assert_eq!(result.unwrap().units(), vec![&3]);
    }

    #[test]
    fn test_transcriber_error() {
        let source = vec![1, 2, 3, 4, 5];
        let result = transcriber::<u8, u8, &str>(source, |cursor, _| {
            cursor.next();
            cursor.next();

            Err("error")
        });

        assert_eq!(result.unwrap_err().tick_buffer, vec![1, 2]);
    }

    #[test]
    fn test_transcriber_completed() {
        let source = vec![1, 2, 3, 4, 5];
        let result = transcriber::<u8, u8, &str>(source, |_, unit| {
            Ok(Some(*unit))
        });

        assert_eq!(result.unwrap().completed, true);
    }

    #[test]
    fn test_transcriber_units() {
        let source = vec![1, 2, 3, 4, 5];
        let result = transcriber::<u8, u8, &str>(source, |_, unit| {
            if *unit == 3 {
                Ok(Some(*unit))
            } else {
                Ok(None)
            }
        });

        assert_eq!(result.unwrap().units(), vec![&3]);
    }

    #[test]
    fn test_transcriber_units_position() {
        let source = vec![1, 2, 3, 4, 5];
        let transcription = transcriber::<u8, u8, &str>(source, |cursor, unit| {
            if *unit == 3 {
                cursor.next();
                Ok(Some(*unit))
            } else {
                Ok(None)
            }
        });

        let transcription = transcription.expect("transcription should be successful");

        assert_eq!(transcription.result[0].source_position, Some(2));
        assert_eq!(transcription.result[0].source_length, Some(1));
    }

    #[test]
    fn test_transcriber_no_transcription() {
        let source = vec![1, 2, 3, 4, 5];
        let result = transcriber::<u8, u8, &str>(source, |_, _| Ok(None));

        assert_eq!(result.unwrap().units(), vec![&0; 0]);
    }

    #[test]
    fn test_transcriber_multiple_transcriptions() {
        let source = vec![1, 2, 3, 4, 5];
        let result = transcriber::<u8, u8, &str>(source, |_, unit| {
            if *unit == 2 || *unit == 4 {
                Ok(Some(*unit))
            } else {
                Ok(None)
            }
        });

        assert_eq!(result.unwrap().units(), vec![&2, &4]);
    }

    #[test]
    fn test_transcriber_error_position() {
        let source = vec![1, 2, 3, 4, 5];
        let result = transcriber::<u8, u8, &str>(source, |cursor, unit| {
            if *unit == 3 {
                cursor.next();
                cursor.next();

                Err("error")
            } else {
                Ok(Some(*unit))
            }
        });

        let error = result.unwrap_err();
        assert_eq!(error.tick_initial_position, 2);
        assert_eq!(error.cursor_position, 4);
        assert_eq!(error.transcription_buffer.units(), vec![&1, &2]);
    }
}