use lubalia_utils::transcriber::error::TranscriberError;
use lubengine::{data::DataValue, lang::{lexer::{lexer, LexerError}, parser::{error::ParserError, parser}, token::Token}, root::ASTRootItem, vm::VM};

/// Evaluate a source code in the VM
pub fn evaluate_code(vm: &mut VM, code: String) -> Result<Option<DataValue>, EvaluationError> {
    // Get the tokens from the source code
    let tokens = lexer(code).map_err(EvaluationError::LexerError)?;

    // Parse the tokens into an AST
    let tree = parser(tokens).map_err(EvaluationError::ParserError)?;

    // Get all the root-nodes from the AST
    let program: Vec<_> = tree.units().into_iter().cloned().collect();

    // Evaluate the program & return the result
    Ok(vm.evaluate(program))
}

/// An error during the evaluation process of a code
#[derive(Debug)]
pub enum EvaluationError {
    /// An error during the lexing process of the code to evaluate
    LexerError(LexerError),

    /// An error during the parsing process of the code to evaluate
    ParserError(TranscriberError<Token, ASTRootItem, ParserError>),
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::LexerError(e) => write!(f, "Lexer error:\n{}", e),
            EvaluationError::ParserError(e) => write!(f, "Parser error:\n{}", e),
        }
    }
}