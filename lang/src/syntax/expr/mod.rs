use literal::LiteralExpression;

use crate::parser::ParserCursor;

pub mod literal;

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Literal(LiteralExpression),
    Identifier,
    Binary,
}

impl ExpressionNode {
    pub fn parse(cursor: &mut ParserCursor) {
        todo!()
    }

    pub fn parse_terminal() {

    }
}
