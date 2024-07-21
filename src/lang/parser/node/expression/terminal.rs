use crate::{
    lang::{parser::{data::DataValue, error::ParserError, node::{statement::{ASTStatement, StatementNode}, Node}}, token::{Token, TokenSymbol}},
    utils::transcriber::cursor::TranscriberCursor, vm::VMTick
};

use super::ExpressionNode;

/// An expression which evaluated result doesn't need manipulation
#[derive(Debug, Clone)]
pub enum TerminalExpression {
    /// A value provided in the code
    Literal(DataValue),

    /// A reference to a variable (thorugh its name)
    VarRef(String),

    /// A scope (a block of code)
    StatementResult(Box<ASTStatement>),

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
            Some(Token::Symbol(TokenSymbol::Underscore)) => Ok(Some(Self::LastValue)),
            _ => {
                cursor.back();
                ASTStatement::transcribe(cursor).map(|o| o.map(|stmt| Self::StatementResult(Box::new(stmt))))
            }
        }
    }
}

impl ExpressionNode for TerminalExpression {
    /// Evaluate the expression and return its value
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::VarRef(varname) => tick.get_context().get(varname.clone()).cloned().unwrap_or_default(),
            Self::StatementResult(statement) => statement.execute(tick).unwrap_or_default(),
            Self::LastValue => tick.vm.last_value.clone()
        }
    }
}