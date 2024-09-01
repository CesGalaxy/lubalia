use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::token::{keyword::TokenLangKeyword, literal::TokenLiteral, Token};

use super::error::TokenizerError;

/// Transcribe an string literal (between double quotes).
pub fn transcribe_string(cursor: &mut TranscriberCursor<char>) -> Result<Option<Token>, TranscriptionException<TokenizerError>> {
    let opening = cursor.consume().ok_or(TokenizerError::UnexpectedEnd).map_err(TranscriptionException::Error)?;

    if opening != &'"' {
        return Err(TranscriptionException::Error(TokenizerError::UnexpectedSymbol(*opening, Some("\""))));
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

/// Transcribe a built-in or custom keyword.
pub fn transcribe_keyword(cursor: &mut TranscriberCursor<char>) -> Result<Option<Token>, TranscriptionException<TokenizerError>> {
    let mut keyword = String::new();

    let initial_char = cursor.consume().ok_or(TokenizerError::UnexpectedEnd).map_err(TranscriptionException::Error)?;

    if !initial_char.is_ascii_alphabetic() && initial_char != &'_' {
        return Err(TranscriptionException::Error(TokenizerError::UnexpectedSymbol(*initial_char, Some("keyword:initial /[a-zA-Z_]/"))));
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
        Ok(Some(Token::Keyword(keyword)))
    } else {
        Ok(Some(Token::Identifier(keyword)))
    }
}

/// Transcribe a number literal.
pub fn transcribe_number(cursor: &mut TranscriberCursor<char>) -> Result<Option<Token>, TranscriptionException<TokenizerError>> {
    let mut literal = String::new();

    let initial_char = cursor.consume().ok_or(TokenizerError::UnexpectedEnd).map_err(TranscriptionException::Error)?;

    if !initial_char.is_numeric() {
        return Err(TranscriptionException::Error(TokenizerError::UnexpectedSymbol(*initial_char, Some("number:initial /[0-9]/"))));
    } else {
        literal.push(*initial_char);
    }

    while let Some(c) = cursor.peek() {
        if c.is_numeric() {
            literal.push(*c);
            cursor.next();
        } else if c == &'.' {
            if cursor.peek_next().is_some_and(|c| c.is_numeric()) {
                literal.push(*c);
                cursor.next();
            } else {
                break;
            }
        } else if c == &'_' {
            cursor.next();
        } else {
            break;
        }
    }

    literal.parse().map_err(|_| TranscriptionException::Error(TokenizerError::ErrorParsingNumber(literal))).map(TokenLiteral::Number).map(Token::Literal).map(Some)
}

/// Transcribes a tag (ex: #name)
pub fn transcribe_tag(cursor: &mut TranscriberCursor<char>) -> Result<Option<Token>, TranscriptionException<TokenizerError>> {
    let initial_char = cursor.consume().ok_or(TokenizerError::UnexpectedEnd).map_err(TranscriptionException::Error)?;

    if initial_char != &'#'
    {
        return Err(TranscriptionException::Error(TokenizerError::UnexpectedSymbol(*initial_char, Some("tag:initial '#'"))));
    }

    let mut tag = String::new();

    while let Some(c) = cursor.peek() {
        if c.is_ascii_alphanumeric() || c == &'_' {
            tag.push(*c);
            cursor.next();
        } else {
            break;
        }
    }

    Ok(Some(Token::Tag(tag)))
}

/// Transcribes a char (ex 'a')
pub fn transcribe_char(cursor: &mut TranscriberCursor<char>) -> Result<Option<Token>, TranscriptionException<TokenizerError>> {
    let initial_char = cursor.consume().ok_or(TokenizerError::UnexpectedEnd).map_err(TranscriptionException::Error)?;

    if initial_char != &'\''
    {
        return Err(TranscriptionException::Error(TokenizerError::UnexpectedSymbol(*initial_char, Some("char:initial /'/"))));
    }

    let character = cursor.consume().ok_or(TokenizerError::UnexpectedEnd).map_err(TranscriptionException::Error)?;

    if cursor.consume() != Some(&'\'') {
        return Err(TranscriptionException::Error(TokenizerError::UnexpectedEnd));
    }

    Ok(Some(Token::Literal(TokenLiteral::Character(*character))))
}
