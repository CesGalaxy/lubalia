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

use colored::Colorize;

/// Transcribe a vec of iUnits into a vec of oUnits.
/// Create an iteration over the iUnits for transcribing them to oUnits
pub fn transcriber<SourceUnit: Clone, ResultUnit: std::fmt::Debug, Error: std::fmt::Display>(
    source: Vec<SourceUnit>,
    tick: impl Fn(&mut TranscriberCursor<SourceUnit>, &SourceUnit) -> Result<Option<ResultUnit>, Error>,
) -> Result<Vec<ResultUnit>, TranscriberError<SourceUnit, ResultUnit, Error>> {
    let mut cursor = TranscriberCursor::new(&source);
    let mut result: Vec<ResultUnit> = vec![];

    while let Some(tick_initial_unit) = source.get(cursor.pos) {
        let tick_initial_position = cursor.pos;

        let tick_result = tick(&mut cursor, tick_initial_unit).map_err(|err| TranscriberError {
            tick_initial_position,
            tick_buffer: source[tick_initial_position..cursor.pos].to_vec(),
            cursor_position: cursor.pos,
            buffer: result,
            error: err,
        });

        if let Some(unit) = tick_result? {
            result.push(unit);
        }

        if tick_initial_position == cursor.pos {
            cursor.next();
        }
    }

    Ok(result)
}

/// A set of tools for moving through a vector with a cursor.
pub struct TranscriberCursor<'a, SourceUnit> {
    pub pos: usize,
    pub source: &'a Vec<SourceUnit>,
}

impl<'a, SourceUnit> TranscriberCursor<'a, SourceUnit> {
    /// Create a new cursor for the given source
    pub fn new(source: &'a Vec<SourceUnit>) -> Self {
        Self { pos: 0, source }
    }

    /// Move forward the cursor
    pub fn next(&mut self) {
        self.pos += 1;
    }

    /// Move backward the cursor
    pub fn back(&mut self) {
        self.pos -= 1;
    }

    /// Get the unit at the cursor position
    pub fn peek(&self) -> Option<&SourceUnit> {
        self.source.get(self.pos)
    }

    /// Get the unit at the cursor position and move the cursor forward
    pub fn consume(&mut self) -> Option<&SourceUnit> {
        let unit = self.peek();
        self.next();
        unit
    }

    /// Check if the cursor is outside the source (cursor position >= source length)
    pub fn is_overflow(&self) -> bool {
        self.pos >= self.source.len()
    }
}

#[derive(Debug)]
pub struct TranscriberError<SourceUnit: Clone, ResultUnit, Error: std::fmt::Display> {
    tick_initial_position: usize,
    tick_buffer: Vec<SourceUnit>,
    cursor_position: usize,
    buffer: Vec<ResultUnit>,
    error: Error,
}

impl<S: std::fmt::Debug + Clone, R, E: std::fmt::Display> std::fmt::Display for TranscriberError<S, R, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (at position {})\n", "Transcriber Error".red().bold(), self.cursor_position.to_string().yellow().bold())?;
        write!(f, "\t{}\n", self.error)?;
        write!(f, "\tBuffer (starts at {}): {:?}", self.tick_initial_position.to_string().yellow().bold(), self.tick_buffer)
    }
}
