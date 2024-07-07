use crate::lang::lexer::token::Token;

use super::machine::ParsingMachine;

#[derive(Debug)]
pub enum ExcpectedToken {
    Keyword(&'static str),
    Symbol(&'static str),
    Literal(&'static str),
    #[allow(dead_code)]
    Unknown(Option<&'static str>)
}

#[derive(Debug)]
pub enum ParserException {
    TokenExpected(ExcpectedToken),
    InvalidToken(Token, Box<ParserError>)
}

#[derive(Debug)]
pub struct ParserError(ParserException, usize);

impl ParsingMachine {
    pub fn except(&self, exception: ParserException) -> ParserError {
        ParserError(exception, self.pos)
    }
}