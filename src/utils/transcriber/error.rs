use colored::Colorize;

use super::result::Transcription;

/// An error during the transcription process
#[derive(Debug)]
pub struct TranscriberError<'a, SourceUnit: Clone, ResultUnit, Error: std::fmt::Display> {
    /// The cursor position when the falied tick started
    pub tick_initial_position: usize,

    /// The amount of source units the tick consumed before the error
    pub tick_buffer: Vec<SourceUnit>,

    /// The cursor position at the time of the error
    pub cursor_position: usize,

    /// The uncompleted transcription before the time of the error
    pub transcription_buffer: Transcription<'a, SourceUnit, ResultUnit>,

    /// The error that occured
    pub error: Error,
}

impl<S: std::fmt::Debug + Clone, R, E: std::fmt::Display> std::fmt::Display for TranscriberError<'_, S, R, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (at position {})\n", "Transcriber Error".red().bold(), self.cursor_position.to_string().yellow().bold())?;
        write!(f, "\t{}\n", self.error)?;
        write!(f, "\tBuffer (starts at {}): {:?}", self.tick_initial_position.to_string().yellow().bold(), self.tick_buffer)
    }
}