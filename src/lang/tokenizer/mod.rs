use error::TokenizerError;

use crate::utils::transcriber::{cursor::TranscriberCursor, result::TranscriptionResult, transcriber};

use super::token::{keyword::TokenLangKeyword, literal::TokenLiteral, symbol::TokenSymbol, Token};

pub mod error;

/// Returns a vector of tokens from the given code.
/// 
/// # Panics
/// 
/// Panics if there is an unexcepted error (not related with the code).
pub fn tokenizer(code: String) -> TranscriptionResult<char, Token, TokenizerError> {
    let code_len = code.len();

    let mut transcription = transcriber(code.chars().collect(), tokenizer_tick)?;

    transcription.push(Token::Symbol(TokenSymbol::EOL), Some(code_len), None);
    transcription.push(Token::Symbol(TokenSymbol::EOF), Some(code_len), None);

    Ok(transcription)
}

fn tokenizer_tick(cursor: &mut TranscriberCursor<char>, initial_unit: &char) -> Result<Option<Token>, TokenizerError> {
    cursor.next();

    match initial_unit {
        // Ignore this
        ' ' | '\t' | '\r' => Ok(None),

        // Strings
        '"' => Ok(Some(Token::Literal(TokenLiteral::String({
            let mut literal = String::new();
    
            while let Some(c) = cursor.consume() {
                if c == &'"' {
                    break;
                }
    
                literal.push(*c);
            }
    
            literal
        })))),

        // Keywords
        // TODO: What about keyword starting with two underscores?
        _ if initial_unit.is_ascii_alphabetic() || (initial_unit == &'_' && cursor.peek().is_some_and(char::is_ascii_alphanumeric)) => Ok(Some({
            let mut keyword = String::from(*initial_unit);
    
            while let Some(c) = cursor.peek() {
                if !c.is_ascii_alphanumeric() || c != &'_' {
                    break;
                }
    
                keyword.push(*c);
                cursor.next();
            }

            if let Some(keyword) = TokenLangKeyword::from_string(&keyword) {
                Token::LangKeyword(keyword)
            } else {
                Token::CustomKeyword(keyword)
            }
        })),

        // Numbers
        _ if initial_unit.is_numeric() => Ok(Some(Token::Literal(TokenLiteral::Number({
            let mut literal = String::from(*initial_unit);
    
            while let Some(c) = cursor.peek() {
                if c.is_numeric() || c == &'.' {
                    literal.push(*c);
                } else if c != &'_' {
                    break;
                }
    
                
                cursor.next();
            }
    
            literal.parse().or_else(|_| Err(TokenizerError::ErrorParsingNumber(literal)))?
        })))),

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