pub mod terminal;
pub mod binary;

use crate::{
    lang::{parser::{data::DataValue, error::ParserError}, token::Token},
    utils::transcriber::cursor::TranscriberCursor, vm::VMTick
};

use super::Node;

#[derive(Debug, Clone)]
pub enum ASTExpression {
    Terminal(terminal::TerminalExpression),
    Binary(binary::BinaryExpression)
}

pub trait ExpressionNode: Node {
    fn evaluate(&self, tick: &mut VMTick) -> DataValue;
}

impl Node for ASTExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTExpression>, ParserError> {
        match cursor.peek_next() {
            Some(Token::Symbol(_)) => binary::BinaryExpression::transcribe(cursor).map(|bexpr| bexpr.map(ASTExpression::Binary)),
            _ => Ok(
                terminal::TerminalExpression::transcribe(cursor)
                    .unwrap_or(None)
                    .map(ASTExpression::Terminal)
            )
        }
    }
}

impl ExpressionNode for ASTExpression {
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        let result = match self {
            ASTExpression::Terminal(expr) => expr.evaluate(tick),
            ASTExpression::Binary(expr) => expr.evaluate(tick)
        };

        tick.vm.last_value = result.clone();
        
        result
    }
}