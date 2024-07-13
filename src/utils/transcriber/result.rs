use super::error::TranscriberError;

/// After transcribing, the result can be a `TranscriberError` or a completed `Transcription`
pub type TranscriptionResult<'a, SourceUnit, ResultUnit, Error> = Result<Transcription<'a, SourceUnit, ResultUnit>, TranscriberError<'a, SourceUnit, ResultUnit, Error>>;

/// The result of a successful transcription (can check if it's `completed`)
#[derive(Debug, PartialEq)]
pub struct Transcription<'a, SourceUnit, ResultUnit> {
    /// The original data that was transcribed, ITUs can make references to it
    /// for showing where they come from.
    pub result: Vec<IdentifiedTranscriptionUnit<'a, SourceUnit, ResultUnit>>,

    /// The transcribed ITUs.
    pub source: Vec<SourceUnit>,

    /// Is set to `true` when the transcription is completed.
    pub completed: bool,
}

impl<SourceUnit, ResultUnit> Transcription<'_, SourceUnit, ResultUnit> {
    /// Create a new empty (uncompleted) transcription for a given source
    pub fn new(source: Vec<SourceUnit>) -> Self {
        Self { result: vec![], source, completed: false }
    }

    /// Identify a new transcribed unit & add it to the result
    pub fn push(&mut self, unit: ResultUnit, initial_position: Option<usize>, current_position: Option<usize>) {
        self.result.push(IdentifiedTranscriptionUnit {
            value: unit,
            // source: &self.source,
            transcription: &self,
            source_position: initial_position,
            source_length: current_position.map(|c| initial_position.map(|i| c - i).unwrap_or(0)),
        });
    }

    /// Get all units
    pub fn units(&self) -> Vec<ResultUnit> {
        self.result.iter().map(|unit| unit.value).collect()
    }
}

/// A unit with extra data for localizing it's source in the transcription source. (aka ITU)
#[derive(Debug, PartialEq)]
pub struct IdentifiedTranscriptionUnit<'a, SourceUnit, ResultUnit> {
    /// THe transcribed unit
    pub value: ResultUnit,

    /// A reference to the transcription which source this unit comes from.
    pub transcription: &'a Transcription<'a, SourceUnit, ResultUnit>,

    /// The position of the source unit in the transcription source.
    /// If `None`, the unit is not localized.
    pub source_position: Option<usize>,

    /// The amount of source units in the transcription source that this unit tooks.
    /// If `None`, the unit is not localized.
    pub source_length: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription() {
        let source = String::from("one two three").chars().collect();
        let mut transcription = Transcription::<'static, char, usize>::new(source);

        transcription.push(1, Some(0), Some(2));
        transcription.push(2, Some(4), Some(7));
        transcription.push(3, Some(8), Some(13));

        assert_eq!(transcription.units(), vec![1, 2, 3], "transcription units should be [1, 2, 3]");
        assert_eq!(transcription.result.iter().map(|itu| itu.source_position).collect::<Vec<_>>(), vec![Some(0), Some(4), Some(8)], "testing units' source positions");
        assert_eq!(transcription.result.iter().map(|itu| itu.source_length).collect::<Vec<_>>(), vec![Some(2), Some(3), Some(5)], "testing units' source lengths");

        assert_eq!(transcription.result[0].value, 1, "testing unit 0 value");
        assert_eq!(transcription.result[1].value, 2, "testing unit 1 value");
        assert_eq!(transcription.result[2].value, 3, "testing unit 2 value");

        assert_eq!(transcription.result[0].transcription, &transcription, "testing unit 2 value");
    }
}