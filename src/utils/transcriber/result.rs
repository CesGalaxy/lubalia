use super::error::TranscriberError;

pub type TranscriptionResult<'a, SourceUnit, ResultUnit, Error> = Result<Transcription<'a, SourceUnit, ResultUnit>, TranscriberError<'a, SourceUnit, ResultUnit, Error>>;

#[derive(Debug)]
pub struct Transcription<'a, SourceUnit, ResultUnit> {
    pub result: Vec<IdentifiedTranscriptionUnit<'a, SourceUnit, ResultUnit>>,
    pub source: Vec<SourceUnit>,
}

impl<SourceUnit, ResultUnit> Transcription<'_, SourceUnit, ResultUnit> {
    pub fn new(source: Vec<SourceUnit>) -> Self {
        Self { result: vec![], source }
    }

    pub fn push(&mut self, unit: ResultUnit, initial_position: Option<usize>, current_position: Option<usize>) {
        self.result.push(IdentifiedTranscriptionUnit {
            data: unit,
            // source: &self.source,
            transcription: &self,
            source_position: initial_position,
            source_length: current_position.map(|c| initial_position.map(|i| c - i).unwrap_or(0)),
        });
    }

    pub fn units(&self) -> Vec<ResultUnit> {
        self.result.iter().map(|unit| unit.data).collect()
    }
}

#[derive(Debug)]
pub struct IdentifiedTranscriptionUnit<'a, SourceUnit, ResultUnit> {
    pub data: ResultUnit,
    // pub source: &'a Vec<SourceUnit>,
    pub transcription: &'a Transcription<'a, SourceUnit, ResultUnit>,
    pub source_position: Option<usize>,
    pub source_length: Option<usize>,
}

// impl<SourceUnit, ResultUnit> From<IdentifiedTranscriptionUnit<'_, SourceUnit, ResultUnit>> for ResultUnit {
//     fn from(unit: IdentifiedTranscriptionUnit<'_, SourceUnit, ResultUnit>) -> Self {
//         unit.data
//     }
// }