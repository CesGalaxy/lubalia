use crate::lang::lexer::token::Token;

use super::{machine::ParsingMachine, ParserError};

#[derive(Debug)]
pub enum ExcpectedToken {
    Keyword(&'static str),
    Symbol(&'static str),
    Literal(&'static str),
    #[allow(dead_code)]
    Unknown(Option<&'static str>)
}

#[derive(Debug)]
pub enum ParsingMachineException {
    TokenExpected(ExcpectedToken),
    InvalidToken(Token, Box<ParsingMachineError>)
}

#[derive(Debug)]
pub struct ParsingMachineError(ParsingMachineException, usize);

impl From<ParsingMachineError> for ParserError {
    fn from(error: ParsingMachineError) -> Self {
        ParserError::ParsingMachineError(error)
    }
}

impl ParsingMachine {
    pub fn except(&self, exception: ParsingMachineException) -> ParsingMachineError {
        ParsingMachineError(exception, self.pos)
    }
}