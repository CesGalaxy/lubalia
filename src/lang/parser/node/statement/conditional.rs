use crate::{lang::{parser::{data::DataValue, error::ParserError, node::{expression::{ASTExpression, ExpressionNode}, ASTNode, Node}}, token::Token}, utils::transcriber::cursor::TranscriberCursor, vm::VMTick};

use super::StatementNode;

#[derive(Debug, Clone)]
pub struct ConditionalStatement {
    pub condition: Box<ASTExpression>,
    pub then_branch: Box<ASTNode>,
    pub else_branch: Option<Box<ASTNode>>,
}

impl Node for ConditionalStatement {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized {
        if cursor.consume() != Some(&Token::Keyword("if".to_string())) {
            return Err(ParserError::Expected("start@conditional <keyword:if> 'if'".to_string()));
        }

        let condition = ASTExpression::transcribe(cursor)?.ok_or(ParserError::Expected("condition@conditional <expr>".to_string()))?;

        let then_branch = ASTNode::transcribe(cursor)?.ok_or(ParserError::Expected("then@conditional <node>".to_string()))?;

        let else_branch = if cursor.consume() == Some(&Token::Keyword("else".to_string())) {
            ASTNode::transcribe(cursor)?
        } else {
            None
        };

        Ok(Some(Self {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new)
        }))
    }
}

impl StatementNode for ConditionalStatement {
    fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        if self.condition.evaluate(tick).into() {
            self.then_branch.execute(tick)
        } else if let Some(else_branch) = &self.else_branch {
            else_branch.execute(tick)
        } else {
            None
        }
    }
}