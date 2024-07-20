use crate::{
    lang::{
        parser::{data::DataValue, error::ParserError, node::{structures::scope::ScopeStruct, Node}},
        token::{Token, TokenSymbol}
    },
    utils::transcriber::cursor::TranscriberCursor, vm::VMTick
};

use super::ExpressionNode;

#[derive(Debug, Clone)]
pub enum TerminalExpression {
    Literal(DataValue),
    VarRef(String),
    Scope(ScopeStruct),
    LastValue
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
            Some(Token::Symbol(TokenSymbol::Underscore)) => Ok(Some(Self::LastValue)),
            _ => Err(ParserError::Expected("<expr:terminal>".to_string()))
        }
    }
}

impl ExpressionNode for TerminalExpression {
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::VarRef(varname) => tick.get_context().get(varname.clone()).cloned().unwrap_or_default(),
            Self::Scope(_) => DataValue::Null,
            Self::LastValue => tick.vm.last_value.clone()
        }
    }
}