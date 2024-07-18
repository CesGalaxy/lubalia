use crate::{
    lang::{
        parser::{data::DataValue, error::ParserError, node::{structures::scope::ScopeStruct, Node}},
        token::{Token, TokenSymbol}
    },
    utils::transcriber::cursor::TranscriberCursor
};

#[derive(Debug, Clone)]
pub enum TerminalExpression {
    Literal(DataValue),
    VarRef(String),
    Scope(ScopeStruct)
}

impl Node for TerminalExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<TerminalExpression>, ParserError> {
        match cursor.consume() {
            Some(Token::Literal(literal)) => Ok(Some(Self::Literal(literal.clone().into()))),
            Some(Token::Keyword(varname)) => match varname.as_str() {
                "true" => Ok(Some(Self::Literal(DataValue::Boolean(true)))),
                "false" => Ok(Some(Self::Literal(DataValue::Boolean(false)))),
                "null" => Ok(Some(Self::Literal(DataValue::Null))),
                _ => Ok(Some(Self::VarRef(varname.clone())))
            },
            Some(Token::Symbol(TokenSymbol::BraceOpen)) => {
                cursor.back();
                ScopeStruct::transcribe(cursor).map(|scope| scope.map(Self::Scope))
            },
            _ => Err(ParserError::Expected("<expr:terminal>".to_string()))
        }
    }
}