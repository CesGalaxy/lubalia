use crate::{lang::{parser::{data::DataValue, error::ParserError, node::Node}, token::Token}, utils::transcriber::cursor::TranscriberCursor};

#[derive(Debug, Clone)]
pub enum TerminalExpression {
    Literal(DataValue),
    VarRef(String)
}

impl Node for TerminalExpression {
    fn transcribe(_cursor: &mut TranscriberCursor<Token>, initial_token: &Token) -> Result<Option<TerminalExpression>, ParserError> {
        match cursor.consume() {
            Token::Literal(literal) => Ok(Some(TerminalExpression::Literal(literal.clone().into()))),
            Token::Keyword(varname) => match varname.as_str() {
                "true" => Ok(Some(TerminalExpression::Literal(DataValue::Boolean(true)))),
                "false" => Ok(Some(TerminalExpression::Literal(DataValue::Boolean(false)))),
                "null" => Ok(Some(TerminalExpression::Literal(DataValue::Null))),
                _ => Ok(Some(TerminalExpression::VarRef(varname.clone())))
            },
            _ => Err(ParserError::Expected("<expr:terminal>".to_string()))
        }
    }
}