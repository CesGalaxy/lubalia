#[derive(Debug)]
pub enum TokenizerError {
    UnexcepedSymbolAtKeyword(String, char),
    UnknownCharacter(char),
    ErrorParsingNumber(String),
}