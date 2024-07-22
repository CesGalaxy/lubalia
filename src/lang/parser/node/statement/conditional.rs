use crate::{lang::{parser::{data::DataValue, error::ParserError, node::{expression::{ASTExpression, ExpressionNode}, ASTNode, Node}}, token::Token}, utils::transcriber::cursor::TranscriberCursor, vm::VMTick};

use super::{scope::ScopeStruct, StatementNode};

#[derive(Debug, Clone)]
pub struct ConditionalStatement {
    pub condition: Box<ASTExpression>,
    pub then_branch: ScopeStruct,
    pub else_branch: Option<ScopeStruct>,
}

impl Node for ConditionalStatement {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized {
        if cursor.consume() != Some(&Token::Keyword("if".to_string())) {
            return Err(ParserError::Expected("start@conditional <keyword:if> 'if'".to_string()));
        }

        let condition = ASTExpression::transcribe(cursor)?.ok_or(ParserError::Expected("condition@conditional <expr>".to_string()))?;

        let then_branch = ScopeStruct::transcribe(cursor)?.ok_or(ParserError::Expected("then_branch@conditional <node>".to_string()))?;

        let else_branch = if cursor.consume() == Some(&Token::Keyword("else".to_string())) {
            Some(ScopeStruct::transcribe(cursor)?.ok_or(ParserError::Expected("else_branch@conditional <node>".to_string()))?)
        } else {
            None
        };

        Ok(Some(Self {
            condition: Box::new(condition),
            then_branch: then_branch,
            else_branch: else_branch
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

impl std::fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.condition, self.then_branch)?;
        if let Some(else_branch) = &self.else_branch {
            write!(f, " else {}", else_branch)?;
        }
        Ok(())
    }
}