use token::Token;
use tokenizer::{tokenizer, TokenizerError};
use linter::{linter, LinterError};

pub mod token;
mod tokenizer;
mod linter;
mod display;

/// Converts the code into a vector of tokens, then
/// checks for errors or warning (linter) and returns
/// the result or the first error found.
/// 
/// # Panics
/// 
/// Panics if there is an unexcepted error (not related with the code).
pub fn lexer(code: String) -> Result<Vec<Token>, LexerError> {
    let tokens = tokenizer(code)?;

    let linter_error = linter(&tokens);

    if let Some(error) = linter_error {
        Err(LexerError::LinterError(error))
    } else {
        Ok(tokens)
    }
}

#[derive(Debug)]
pub enum LexerError {
    TokenizerError(TokenizerError),
    LinterError(LinterError)
}
