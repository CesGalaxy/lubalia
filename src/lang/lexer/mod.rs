use token::Token;
use tokenizer::{tokenizer, TokenizerError};
use linter::{linter, LinterError};

pub mod token;
mod tokenizer;
mod linter;
mod display;

pub fn lexer(code: String) -> Result<Vec<Token>, LexerResult> {
    let tokens = tokenizer(code)?;

    let linter_error = linter(&tokens);

    if let Some(error) = linter_error {
        Err(LexerResult::LinterError(error))
    } else {
        Ok(tokens)
    }
}

#[derive(Debug)]
pub enum LexerResult {
    Ok,
    TokenizerError(TokenizerError),
    LinterError(LinterError)
}
