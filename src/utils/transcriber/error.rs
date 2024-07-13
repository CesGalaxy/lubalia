use colored::Colorize;

use super::result::Transcription;

#[derive(Debug)]
pub struct TranscriberError<'a, SourceUnit: Clone, ResultUnit, Error: std::fmt::Display> {
    pub tick_initial_position: usize,
    pub tick_buffer: Vec<SourceUnit>,
    pub cursor_position: usize,
    pub buffer: Transcription<'a, SourceUnit, ResultUnit>,
    pub error: Error,
}

impl<S: std::fmt::Debug + Clone, R, E: std::fmt::Display> std::fmt::Display for TranscriberError<'_, S, R, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (at position {})\n", "Transcriber Error".red().bold(), self.cursor_position.to_string().yellow().bold())?;
        write!(f, "\t{}\n", self.error)?;
        write!(f, "\tBuffer (starts at {}): {:?}", self.tick_initial_position.to_string().yellow().bold(), self.tick_buffer)
    }
}