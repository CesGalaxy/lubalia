// pub trait Transcriber {
//     /// Input Unit (iUnit)
//     type SourceUnit;
//     /// Output Unit (oUnit)
//     type ResultUnit;
//     /// Error
//     type Error;
//
//     /// Transcribe a vec of iUnits into a vec of oUnits.
//     /// Create an iteration over the iUnits for transcribing them to oUnits
//     fn transcribe(source: Vec<Self::SourceUnit>) -> Result<Vec<Self::ResultUnit>, TranscriberError<Self::Error>> {
//         let mut cursor = TranscriberCursor::new(&source);
//         let mut result: Vec<Self::ResultUnit> = vec![];
//
//         while let Some(initial_unit) = source.get(cursor.pos) {
//             let tick_initial_position = cursor.pos;
//
//             let tick_result = Self::tick(&mut cursor, initial_unit).map_err(|err| TranscriberError {
//                 tick_initial_position,
//                 cursor_position: cursor.pos,
//                 error: err
//             });
//
//             result.push(tick_result?);
//
//             if tick_initial_position == cursor.pos {
//                 cursor.next();
//             }
//         }
//
//         Ok(result)
//     }
//
//     /// Each tick, starts at the initial iUnit of the current oUnit, and must end with the initial iUnit of the next oUnit.
//     fn tick(cursor: &mut TranscriberCursor<Self::SourceUnit>, initial_unit: &Self::SourceUnit) -> Result<Self::ResultUnit, Self::Error>;
// }

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
) -> TranscriptionResult<'static, SourceUnit, ResultUnit, Error> {
    let mut transcription = Transcription::new(source);
    let mut cursor = TranscriberCursor::new(&transcription.source);

    while let Some(tick_initial_unit) = cursor.peek() {
        let tick_initial_position = cursor.pos;

        let tick_result = tick(&mut cursor, tick_initial_unit).map_err(|err| TranscriberError {
            tick_initial_position,
            tick_buffer: source[tick_initial_position..cursor.pos].to_vec(),
            cursor_position: cursor.pos,
            buffer: transcription,
            error: err,
        });

        if let Some(unit) = tick_result? {
            transcription.push(unit, Some(tick_initial_position), Some(cursor.pos));
        }

        if tick_initial_position == cursor.pos {
            cursor.next();
        }
    }

    Ok(transcription)
}




