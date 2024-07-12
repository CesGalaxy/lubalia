use colored::Colorize;

use crate::lang::lexer::token::Token;

use super::machine::ParsingMachine;

#[derive(Debug)]
pub enum ExpectedToken {
    Keyword(&'static str),
    Symbol(&'static str),
    Literal(&'static str),
    Expression(Option<&'static str>),
    Unknown(Option<&'static str>)
}

#[derive(Debug)]
pub enum ParserException {
    TokenExpected(ExpectedToken),
    ExpressionExpected(&'static str),
    InvalidToken(Token, Box<ParserError>)
}

#[derive(Debug)]
pub struct ParserError {
    exception: ParserException,
    pos: Option<usize>,
}

impl std::fmt::Display for ExpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedToken::Keyword(keyword) => write!(f, "keyword: {keyword}"),
            ExpectedToken::Symbol(symbol) => write!(f, ""),
            ExpectedToken::Literal(literal) => write!(f, ""),
            _ => write!(f, "a")
        }
    }
}

impl std::fmt::Display for ParserException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserException::TokenExpected(expected) => write!(f, "Expected token: {expected:?}"),
            ParserException::InvalidToken(token, error) => write!(f, "Invalid token: '{token}' (parser error: {error})"),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Exception".red())?;
        write!(f, "{}", self.exception)?;
        write!(f, "{}", self.exception)
    }
}

impl ParsingMachine {
    pub fn err(&self, exception: ParserException) -> ParserError {
        ParserError {
            exception,
            pos: Some(self.pos)
        }
    }
}