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

/// Transcribe a vec of iUnits into a vec of oUnits.
/// Create an iteration over the iUnits for transcribing them to oUnits
pub fn transcriber<SourceUnit, ResultUnit, Error>(
    source: Vec<SourceUnit>,
    tick: impl Fn(&mut TranscriberCursor<SourceUnit>, &SourceUnit) -> Result<Option<ResultUnit>, Error>,
) -> Result<Vec<ResultUnit>, TranscriberError<Error>> {
    let mut cursor = TranscriberCursor::new(&source);
    let mut result: Vec<ResultUnit> = vec![];

    while let Some(initial_unit) = source.get(cursor.pos) {
        let tick_initial_position = cursor.pos;

        let tick_result = tick(&mut cursor, initial_unit).map_err(|err| TranscriberError {
            tick_initial_position,
            cursor_position: cursor.pos,
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

pub struct TranscriberError<Error> {
    tick_initial_position: usize,
    cursor_position: usize,
    error: Error,
}
