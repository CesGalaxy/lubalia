use std::fmt;

use colored::Colorize;

use super::result::IdentifiedTranscriptionUnit;

/// An error during the transcription process
#[derive(Debug)]
pub struct TranscriberError<SourceUnit: Clone, ResultUnit, Error: fmt::Display> {
    /// The cursor position when the falied tick started
    pub tick_initial_position: usize,

    /// The amount of source units the tick consumed before the error
    pub tick_buffer: Vec<SourceUnit>,

    /// The cursor position at the time of the error
    pub cursor_position: usize,

    /// The uncompleted transcription before the time of the error
    pub transcription_buffer: Vec<IdentifiedTranscriptionUnit<ResultUnit>>,

    /// The error that occured
    pub error: Error,
}

impl<SourceUnit: fmt::Debug + Clone, ResultUnit, Error: fmt::Display> fmt::Display for TranscriberError<SourceUnit, ResultUnit, Error> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (at position {})\n", "Transcriber Error".red().bold(), self.cursor_position.to_string().yellow().bold())?;
        write!(f, "\t{}\n", self.error)?;
        write!(f, "\tBuffer (starts at {}): {:?}", self.tick_initial_position.to_string().yellow().bold(), self.tick_buffer)
    }
}