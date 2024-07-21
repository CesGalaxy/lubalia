use crate::{
    lang::{parser::{data::DataValue, error::ParserError, node::Node}, token::{Token, TokenSymbol}},
    utils::transcriber::cursor::TranscriberCursor, vm::VMTick
};

use super::{scope::ScopeStruct, ExpressionNode};

/// An expression which evaluated result doesn't need manipulation
#[derive(Debug, Clone)]
pub enum TerminalExpression {
    /// A value provided in the code
    Literal(DataValue),

    /// A reference to a variable (thorugh its name)
    VarRef(String),

    /// A scope (a block of code)
    Scope(ScopeStruct),

    /// A reference to the last evaluated expression value
    LastValue
}

impl Node for TerminalExpression {
    /// Transcribe a terminal expression (literal, variable reference, scope, etc.)
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
    /// Evaluate the expression and return its value
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::VarRef(varname) => tick.get_context().get(varname.clone()).cloned().unwrap_or_default(),
            Self::Scope(scope) => scope.evaluate(tick),
            Self::LastValue => tick.vm.last_value.clone()
        }
    }
}