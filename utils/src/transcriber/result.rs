use super::error::TranscriptionError;

/// After transcribing, the result can be a `TranscriberError` or a completed `Transcription`
pub type TranscriptionResult<SourceUnit, ResultUnit, Error> = Result<Transcription<SourceUnit, ResultUnit>, TranscriptionError<SourceUnit, ResultUnit, Error>>;

/// The result of a successful transcription (can check if it's `completed`)
#[derive(Debug, PartialEq)]
pub struct Transcription<SourceUnit, ResultUnit> {
    /// The original data that was transcribed, ITUs can make references to it
    /// for showing where they come from.
    pub result: Vec<IdentifiedTranscriptionUnit<ResultUnit>>,

    /// The transcribed ITUs.
    pub source: Vec<SourceUnit>
}

impl<SourceUnit, ResultUnit> Transcription<SourceUnit, ResultUnit> {
    /// Create a new empty (uncompleted) transcription for a given source
    pub fn new(result: Vec<IdentifiedTranscriptionUnit<ResultUnit>>, source: Vec<SourceUnit>) -> Self {
        Self { result, source }
    }

    /// Get all units
    pub fn units(&self) -> Vec<&ResultUnit> {
        self.result.iter().map(|unit| &unit.value).collect()
    }
}

/// A unit with extra data for localizing it's source in the transcription source. (aka ITU)
#[derive(Debug, PartialEq, Clone)]
pub struct IdentifiedTranscriptionUnit<Unit> {
    /// The transcribed unit
    pub value: Unit,

    /// The position of the source unit in the transcription source.
    /// If `None`, the unit is not localized.
    pub source_position: Option<usize>,

    /// The amount of source units in the transcription source that this unit took.
    /// If `None`, the unit is not localized.
    pub source_length: Option<usize>,
}

impl<Unit> IdentifiedTranscriptionUnit<Unit> {
    /// Identify a new transcribed unit & add it to the result
    pub fn new(unit: Unit, initial_position: Option<usize>, current_position: Option<usize>) -> Self {
        Self {
            value: unit,
            source_position: initial_position,
            source_length: current_position.map(|c| initial_position.map(|i| c - i).unwrap_or(0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transcription() {
        let result = vec![
            IdentifiedTranscriptionUnit::new("Result 1", Some(0), Some(5)),
            IdentifiedTranscriptionUnit::new("Result 2", Some(6), Some(11)),
        ];
        let source = vec!["Source 1", "Source 2"];
        let transcription = Transcription::new(result.clone(), source.clone());

        assert_eq!(transcription.result, result);
        assert_eq!(transcription.source, source);
    }

    #[test]
    fn test_units() {
        let result = vec![
            IdentifiedTranscriptionUnit::new("Result 1", Some(0), Some(5)),
            IdentifiedTranscriptionUnit::new("Result 2", Some(6), Some(11)),
        ];
        let source = vec!["Source 1", "Source 2"];
        let transcription = Transcription::new(result.clone(), source.clone());

        let expected_units = vec!["Result 1", "Result 2"];
        assert_eq!(transcription.units().iter().map(|unit| *unit as &str).collect::<Vec<&str>>(), expected_units);
    }

    #[test]
    fn test_identified_transcription_unit() {
        let unit = IdentifiedTranscriptionUnit::new("Result", Some(0), Some(5));

        assert_eq!(unit.value, "Result");
        assert_eq!(unit.source_position, Some(0));
        assert_eq!(unit.source_length, Some(5));
    }
}