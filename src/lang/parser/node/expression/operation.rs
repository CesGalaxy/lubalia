use crate::{lang::{lexer::token::Token, parser::{data::DataValue, exception::{ExcpectedToken, ParsingMachineError, ParsingMachineException}, machine::ParsingMachine, node::{Node, NodeFactory}}}, vm::scope::Scope};

use super::{literal::LiteralExpresionNode, ExpressionNode};

#[derive(Debug)]
pub enum OperationExpressionNode {
    Add(DataValue, DataValue),
    Sub(DataValue, DataValue),
    Mul(DataValue, DataValue),
    Div(DataValue, DataValue)
}

impl ExpressionNode for OperationExpressionNode {
    fn evaluate(&self, _scope: &Scope) -> DataValue {
        match self {
            OperationExpressionNode::Add(a, b) => {
                if let DataValue::Number(n1) = a {
                    if let DataValue::Number(n2) = b {
                        DataValue::Number(n1 + n2)
                    } else {
                        DataValue::Number(0.0)
                    }
                } else {
                    DataValue::Number(0.0)
                }
            }
            _ => DataValue::Number(0.0)
        }
    }
}

impl NodeFactory for OperationExpressionNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParsingMachineError> {
        let n1 = LiteralExpresionNode::from_tokens(m)?;
        
        match m.consume() {
            Some(Token::Symbol(_)) => {
                let n2 = LiteralExpresionNode::from_tokens(m)?;
                Ok(Self::Add(n1.0, n2.0))
            },
            _ => Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Symbol("+"))))
        }
    }
}

impl Node for OperationExpressionNode {}

