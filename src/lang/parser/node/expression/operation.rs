use crate::{lang::{lexer::token::{Token, TokenSymbol}, parser::{data::DataValue, exception::{ExcpectedToken, ParsingMachineError, ParsingMachineException}, machine::ParsingMachine, node::{Node, NodeFactory}}}, vm::scope::Scope};

use super::{literal::LiteralExpresionNode, ExpressionNode};

#[derive(Debug, Clone)]
pub enum OperationExpressionNode {
    Add(DataValue, DataValue),
    Sub(DataValue, DataValue),
    Mul(DataValue, DataValue),
    Div(DataValue, DataValue)
}

impl ExpressionNode for OperationExpressionNode {
    /// Operates the values
    fn evaluate(&self, _scope: &Scope) -> DataValue {
        match self {
            OperationExpressionNode::Add(a, b) => a + b,
            OperationExpressionNode::Sub(a, b) => a - b,
            OperationExpressionNode::Mul(a, b) => a * b,
            OperationExpressionNode::Div(a, b) => a / b,
        }
    }
}

impl NodeFactory for OperationExpressionNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParsingMachineError> {
        // Get the first value
        let n1 = LiteralExpresionNode::from_tokens(m)?.into();
        
        match m.consume() {
            // Check which operation to perform
            Some(Token::Symbol(symbol)) => match symbol {
                TokenSymbol::Plus => Ok(Self::Add(n1, LiteralExpresionNode::from_tokens(m)?.0)),
                TokenSymbol::Minus => Ok(Self::Sub(n1, LiteralExpresionNode::from_tokens(m)?.0)),
                TokenSymbol::Asterisk => Ok(Self::Mul(n1, LiteralExpresionNode::from_tokens(m)?.0)),
                TokenSymbol::Slash => Ok(Self::Div(n1, LiteralExpresionNode::from_tokens(m)?.0)),
                _ => Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Symbol("<operand>"))))
            },
            _ => Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Symbol("<operand>"))))
        }
    }
}

impl Node for OperationExpressionNode {}

impl std::fmt::Display for OperationExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationExpressionNode::Add(a, b) => write!(f, "{} + {}", a, b),
            OperationExpressionNode::Sub(a, b) => write!(f, "{} - {}", a, b),
            OperationExpressionNode::Mul(a, b) => write!(f, "{} * {}", a, b),
            OperationExpressionNode::Div(a, b) => write!(f, "{} / {}", a, b),
        }
    }
}
