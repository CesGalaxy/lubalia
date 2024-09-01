use std::fmt;

use lubalia_utils::transcriber::error::TranscriptionError;

use super::{token::Token, tokenizer::{error::TokenizerError, tokenizer}};

/// Converts the code into a vector of tokens, then
/// checks for errors or warning (linter) and returns
/// the result or the first error found.
///
/// # Panics
///
/// Panics if there is an unexcepted error (not related with the code).
pub fn lexer(code: String) -> Result<Vec<Token>, LexerError> {
    let tokenization = tokenizer(code)?;
    let tokens = tokenization.units().into_iter().cloned().collect();

    Ok(tokens)
}

/// An error during the lexical pparsing process.
///
/// It can be a tokenizer error or a linter error.
#[derive(Debug)]
pub enum LexerError {
    /// An error during the tokenizer (transcribing) process.
    TokenizerError(TranscriptionError<char, Token, TokenizerError>),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenizerError(error) => write!(f, "TokenizerError >> {error}"),
        }
    }
}

impl From<TranscriptionError<char, Token, TokenizerError>> for LexerError {
    fn from(err: TranscriptionError<char, Token, TokenizerError>) -> Self {
        Self::TokenizerError(err)
    }
}
