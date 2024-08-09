/// The lexer works with the source code and transcribes it into a list of tokens (with the tokenizer).
/// Then, the linter checks the tokens for syntax errors.
pub mod lexer;

/// The parser transcribes a list of tokens into an AST.
pub mod parser;

/// The tokenizer transcribes a source code into a list of tokens.
pub mod tokenizer;

/// The token module contains the definition of tokens.
pub mod token;

/// The syntax module contains the definitions of all the AST and their execution.
pub mod syntax;