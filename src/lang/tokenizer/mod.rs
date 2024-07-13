use error::TokenizerError;

use crate::utils::transcriber::{transcriber, TranscriberCursor, TranscriberError};

use super::token::{data::{TokenData, TokenLiteral, TokenSymbol}, Token};

pub mod error;

/// Returns a vector of tokens from the given code.
/// 
/// # Panics
/// 
/// Panics if there is an unexcepted error (not related with the code).
pub fn tokenizer(code: String) -> Result<Vec<Token>, TranscriberError<char, Token, TokenizerError>> {
    let mut tokens = transcriber(code.chars().collect(), tokenize_token)?;

    tokens.push(Token(TokenData::EOL));
    tokens.push(Token(TokenData::EOF));

    Ok(tokens)
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

        Ok(Some(Token(
            TokenData::Keyword(keyword)
        )))
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

        Ok(Some(Token(
            TokenData::Literal(TokenLiteral::Number(literal.parse().or_else(|_| Err(TokenizerError::ErrorParsingNumber(literal)))?))
        )))
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

        Ok(Some(Token(
            TokenData::Literal(TokenLiteral::String(literal))
        )))
    } else {
        if let Some(symbol) = TokenSymbol::from_char(*initial_unit) {
            Ok(Some(Token(
                TokenData::Symbol(symbol)
            )))
        } else {
            match initial_unit {
                ';' => Ok(Some(Token(TokenData::Semicolon))),
                '\n' => Ok(Some(Token(TokenData::EOL))),
                _ => Err(TokenizerError::UnknownCharacter(*initial_unit))
            }
        }
    }
}
