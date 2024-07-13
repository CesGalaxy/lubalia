use linter::{linter, LinterError};

use crate::utils::transcriber::error::TranscriberError;

use super::{token::Token, tokenizer::{error::TokenizerError, tokenizer}};
mod linter;

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

    let linter_error = linter(&tokens);

    if let Some(error) = linter_error {
        Err(LexerError::LinterError(error))
    } else {
        Ok(tokens)
    }
}

/// An error during the lexical pparsing process.
/// 
/// It can be a tokenizer error or a linter error.
#[derive(Debug)]
pub enum LexerError {
    /// An error during the tokenizer (transcribing) process.
    TokenizerError(TranscriberError<char, Token, TokenizerError>),
    
    /// A linter error, usually a missing semicolon.
    LinterError(LinterError)
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenizerError(error) => write!(f, "TokenizerError >> {error}"),
            Self::LinterError(error) => write!(f, "LinterError >> {error}"),
        }
    }
}

impl From<TranscriberError<char, Token, TokenizerError>> for LexerError {
    fn from(err: TranscriberError<char, Token, TokenizerError>) -> Self {
        Self::TokenizerError(err)
    }
}
