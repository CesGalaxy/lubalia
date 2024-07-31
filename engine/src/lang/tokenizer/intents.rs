use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::lang::token::{keyword::TokenLangKeyword, literal::TokenLiteral, Token};

use super::error::TokenizerError;

pub fn transcribe_string(cursor: &mut TranscriberCursor<char>) -> Result<Option<Token>, TokenizerError> {
    let opening = cursor.consume().ok_or(TokenizerError::UnexpectedEnd)?;

    if opening != &'"' {
        return Err(TokenizerError::UnexpectedSymbol(*opening, Some("\"")));
    }

    let mut buffer = String::new();

    while let Some(c) = cursor.consume() {
        if c == &'"' {
            break;
        }

        // TODO: Handle escape characters
        // TODO: Handle new lines
        // TODO: Handle end of file

        buffer.push(*c);
    }

    Ok(Some(Token::Literal(TokenLiteral::String(buffer))))
}

pub fn transcribe_keyword(cursor: &mut TranscriberCursor<char>) -> Result<Token, TokenizerError> {
    let mut keyword = String::new();

    let initial_char = cursor.consume().ok_or(TokenizerError::UnexpectedEnd)?;

    if !initial_char.is_ascii_alphabetic() && initial_char != &'_' {
        return Err(TokenizerError::UnexpectedSymbol(*initial_char, Some("keyword:initial /[a-zA-Z_]/")));
    } else {
        keyword.push(*initial_char);
    }

    while let Some(c) = cursor.peek() {
        if !c.is_ascii_alphanumeric() && c != &'_' {
            break;
        }

        keyword.push(*c);
        cursor.next();
    }

    if let Some(keyword) = TokenLangKeyword::from_string(&keyword) {
        Ok(Token::LangKeyword(keyword))
    } else {
        Ok(Token::CustomKeyword(keyword))
    }
}

pub fn transcribe_number(cursor: &mut TranscriberCursor<char>) -> Result<Token, TokenizerError> {
    let mut literal = String::new();

    let initial_char = cursor.consume().ok_or(TokenizerError::UnexpectedEnd)?;

    if !initial_char.is_numeric() {
        return Err(TokenizerError::UnexpectedSymbol(*initial_char, Some("number:initial /[0-9]/")));
    } else {
        literal.push(*initial_char);
    }

    while let Some(c) = cursor.peek() {
        if c.is_numeric() || c == &'.' || c == &'_' {
            literal.push(*c);
            cursor.next();
        } else {
            break;
        }
    }

    literal.parse().map_err(|_| TokenizerError::ErrorParsingNumber(literal)).map(TokenLiteral::Number).map(Token::Literal)
}