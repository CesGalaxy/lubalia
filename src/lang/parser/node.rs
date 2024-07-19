pub mod expression;
pub mod structures;
pub mod statement;

use expression::{ASTExpression, ExpressionNode};
use statement::{ASTStatement, StatementNode};

use crate::{lang::token::Token, utils::transcriber::cursor::TranscriberCursor, vm::context::Context};

use super::{data::DataValue, error::ParserError};

#[derive(Debug, Clone)]
pub enum ASTNode {
    Expression(ASTExpression),
    Statement(ASTStatement)
}

pub trait Node {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized;
}

impl ASTNode {
    pub fn execute(&self, context: &mut Context) -> Option<DataValue> {
        match self {
            Self::Expression(expr) => Some(expr.evaluate(context)),
            Self::Statement(statement) => {
                statement.execute(context).ok();
                None
            }
        }
    }
}

impl Node for ASTNode {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTNode>, ParserError> {
        match cursor.peek().expect("Expected token") {
            Token::EOL => Ok(None),
            Token::Keyword(_) => ASTStatement::transcribe(cursor).map(|stmt| stmt.map(ASTNode::Statement)),
            _ => ASTExpression::transcribe(cursor).map(|aste| aste.map(ASTNode::Expression))
        }
    }
}