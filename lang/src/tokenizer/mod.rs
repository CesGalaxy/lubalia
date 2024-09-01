use error::TokenizerError;
use intents::{transcribe_char, transcribe_keyword, transcribe_number, transcribe_string, transcribe_tag};
use lubalia_utils::{cursor::CursorNavigation, loop_through::LoopThrough, transcriber::{cursor::TranscriberCursor, error::TranscriptionException, result::{IdentifiedTranscriptionUnit, TranscriptionResult}, transcriber, TranscriberTickResult}};

use super::token::{symbol::TokenSymbol, Token};

pub mod error;
pub mod intents;

pub type TokenizerTickResult = TranscriberTickResult<Token, TokenizerError>;

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

fn tokenizer_tick(cursor: &mut TranscriberCursor<char>, initial_unit: &char) -> TokenizerTickResult {
    match initial_unit {
        ' ' | '\t' | '\r' => Ok(None),

        // Comments
        '/' if cursor.peek_next() == Some(&'/') => {
            cursor.ignore_loop(LoopThrough::UntilEq(&'\n'));
            Ok(None)
        }

        // Inline comments
        '/' if cursor.peek_next() == Some(&'*') => {
            cursor.next();

            while let Some(unit) = cursor.consume() {
                if unit == &'*' && cursor.peek() == Some(&'/') {
                    cursor.next();
                    break;
                }
            }

            Ok(None)
        }

        // Strings
        '"' => transcribe_string(cursor),

        // Tags
        '#' => transcribe_tag(cursor),

        // Characters
        '\'' => transcribe_char(cursor),

        // Keywords
        _ if initial_unit.is_ascii_alphabetic() || (initial_unit == &'_' && cursor.peek().is_some_and(|c| char::is_ascii_alphanumeric(c) || c == &'_')) =>
            transcribe_keyword(cursor),

        // Numbers
        _ if initial_unit.is_numeric() => transcribe_number(cursor),

        // Symbols (or Error if neither)
        _ => if let Some(symbol) = TokenSymbol::from_char(*initial_unit) {
            Ok(Some(Token::Symbol(symbol)))
        } else {
            Err(TranscriptionException::Error(TokenizerError::UnknownCharacter(*initial_unit)))
        }
    }
}

#[cfg(test)]
mod tests;