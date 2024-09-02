pub mod list;

use crate::parser::ParserCursor;

use super::{expr::ExpressionNode, node::{const_declaration::ConstDeclaration, return_value::ReturnValue, var_declaration::VarDeclaration, NodeParsingResult}};

#[derive(Debug)]
pub enum StatementNode {
    ConstDeclaration(ConstDeclaration),
    VariableDeclaration(VarDeclaration),
    Expression(ExpressionNode),
    Return(ReturnValue),
}

impl StatementNode {
    pub fn parse(_cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        todo!()
    }
}
