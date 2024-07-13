use error::TokenizerError;

use crate::utils::transcriber::{cursor::TranscriberCursor, result::TranscriptionResult, transcriber};

use super::token::{Token, TokenLiteral, TokenSymbol};

pub mod error;

/// Returns a vector of tokens from the given code.
/// 
/// # Panics
/// 
/// Panics if there is an unexcepted error (not related with the code).
pub fn tokenizer(code: String) -> TranscriptionResult<'static, char, Token, TokenizerError> {
    let code_len = code.len();

    let mut transcription = transcriber(code.chars().collect(), tokenize_token)?;

    transcription.push(Token::EOL, Some(code_len), None);
    transcription.push(Token::EOF, Some(code_len), None);

    Ok(transcription)
}

fn tokenize_token(cursor: &mut TranscriberCursor<char>, initial_unit: &char) -> Result<Option<Token>, TokenizerError> {
    if initial_unit == &' ' || initial_unit == &'\t' {
        cursor.next();
        Ok(None)
    } else if initial_unit.is_ascii_alphabetic() {
        let mut keyword = String::new();
        keyword.push(*initial_unit);
        cursor.next();

        while let Some(c) = cursor.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }

            keyword.push(*c);
            cursor.next();
        }

        Ok(Some(Token::Keyword(keyword)))
    } else if initial_unit.is_numeric() {
        let mut literal = String::new();
        literal.push(*initial_unit);
        cursor.next();

        while let Some(c) = cursor.peek() {
            if !c.is_numeric() {
                break;
            }

            literal.push(*c);
            cursor.next();
        }

        Ok(Some(Token::Literal(TokenLiteral::Number(literal.parse().or_else(|_| Err(TokenizerError::ErrorParsingNumber(literal)))?))))
    } else if initial_unit == &'"' {
        let mut literal = String::new();
        cursor.next();

        while let Some(c) = cursor.peek() {
            if c == &'"' {
                break;
            }

            literal.push(*c);
            cursor.next();
        }

        Ok(Some(Token::Literal(TokenLiteral::String(literal))))
    } else {
        if let Some(symbol) = TokenSymbol::from_char(*initial_unit) {
            Ok(Some(Token::Symbol(symbol)))
        } else {
            match initial_unit {
                ';' => Ok(Some(Token::Semicolon)),
                '\n' => Ok(Some(Token::EOL)),
                _ => return Err(TokenizerError::UnknownCharacter(*initial_unit)),
            }
        }
    }
}
