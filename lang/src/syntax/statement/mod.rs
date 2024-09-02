pub mod list;

use lubalia_utils::cursor::CursorNavigation;

use crate::{parser::ParserCursor, token::{keyword::TokenLangKeyword, Token}};

use super::{expr::ExpressionNode, node::{const_declaration::ConstDeclaration, return_value::ReturnValue, var_declaration::VarDeclaration, NodeParsingResult}};

#[derive(Debug, Clone)]
pub enum StatementNode {
    ConstDeclaration(ConstDeclaration),
    VariableDeclaration(VarDeclaration),
    Expression(ExpressionNode),
    Return(ReturnValue),
}

impl StatementNode {
    pub fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self>  {
        match cursor.peek() {
            Some(Token::Keyword(TokenLangKeyword::Let)) => Ok(VarDeclaration::parse(cursor)?.map(Self::VariableDeclaration)),
            _ => Ok(ExpressionNode::parse(cursor)?.map(Self::Expression)),
        }
    }
}
