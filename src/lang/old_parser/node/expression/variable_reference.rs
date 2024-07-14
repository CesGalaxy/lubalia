use crate::{
    lang::{
        token::Token,
        old_parser::{
            data::DataValue,
            exception::{ExpectedToken, ParserError, ParserException},
            machine::ParsingMachine,
            node::{Node, NodeFactory}
        }
    },
    vm::context::Context
};

use super::ExpressionNode;

/// A reference (through the name) to a variable of the scope
#[derive(Debug, Clone)]
pub struct VariableReferenceNode(String);

impl Node for VariableReferenceNode {}

impl NodeFactory for VariableReferenceNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        match m.consume() {
            Some(Token::Keyword(varname)) => Ok(Self(varname)),
            _ => Err(m.err(ParserException::TokenExpected(ExpectedToken::Symbol("<varname>"))))
        }
    }
}

impl ExpressionNode for VariableReferenceNode {
    /// Get the variable value from the scope, return 0.0 by default
    fn evaluate(&self, scope: &Context) -> DataValue {
        scope.get(self.0.clone()).map(|value| value.clone()).unwrap_or(DataValue::Number(0.0))
    }
}

impl std::fmt::Display for VariableReferenceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
