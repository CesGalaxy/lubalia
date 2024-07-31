use error::TokenizerError;
use intents::{transcribe_keyword, transcribe_string, transcribe_number};
use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, result::{IdentifiedTranscriptionUnit, TranscriptionResult}, transcriber}};

use super::token::{symbol::TokenSymbol, Token};

pub mod error;
pub mod intents;

/// Returns a vector of tokens from the given code.
/// 
/// # Panics
/// 
/// Panics if there is an unexcepted error (not related with the code).
pub fn tokenizer(code: String) -> TranscriptionResult<char, Token, TokenizerError> {
    let code_len = code.len();

    let mut transcription = transcriber(code.chars().collect(), tokenizer_tick)?;

    transcription.result.push(IdentifiedTranscriptionUnit::new(Token::Symbol(TokenSymbol::EOL), Some(code_len), None));
    transcription.result.push(IdentifiedTranscriptionUnit::new(Token::Symbol(TokenSymbol::EOF), Some(code_len), None));

    Ok(transcription)
}

fn tokenizer_tick(cursor: &mut TranscriberCursor<char>, initial_unit: &char) -> Result<Option<Token>, TokenizerError> {
    match initial_unit {
        ' ' | '\t' | '\r' => Ok(None),

        '"' => transcribe_string(cursor),

        // '\'' => Ok(Some(Token::Literal({
        //     if let Some('\'') = cursor.peek_next() {
        //         TokenLiteral::Character(cursor.consume().unwrap())
        //     }
        // }))),

        // Keywords
        _ if initial_unit.is_ascii_alphabetic() || (initial_unit == &'_' && cursor.peek().is_some_and(|c| char::is_ascii_alphanumeric(c) || c == &'_')) =>
            transcribe_keyword(cursor).map(Some),

        // Numbers
        _ if initial_unit.is_numeric() => transcribe_number(cursor).map(Some),

        // Symbols (or Error if neither)
        _ => if let Some(symbol) = TokenSymbol::from_char(*initial_unit) {
            Ok(Some(Token::Symbol(symbol)))
        } else {
            Err(TokenizerError::UnknownCharacter(*initial_unit))
        }
    }
}

#[cfg(test)]
mod tests;