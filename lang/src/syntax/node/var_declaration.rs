use lubalia_utils::cursor::CursorNavigation;

use crate::{parser::{error::ParserError, ParserCursor}, syntax::expr::ExpressionNode, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}};

use super::NodeParsingResult;

#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub name: String,
    pub value: ExpressionNode,
}

impl VarDeclaration {
    pub fn parse(cursor: &mut ParserCursor) -> NodeParsingResult<Self> {
        cursor.expect(&Token::Keyword(TokenLangKeyword::Let), ParserError::Expected("start@var_declaration <keyword:let>"))?;

        if let Some(Token::Identifier(name)) = cursor.consume() {
            cursor.expect(&Token::Symbol(TokenSymbol::Equal), ParserError::Expected("eq@var_declaration <symbol:eq>"))?;

            let value = ExpressionNode::parse(cursor)?.ok_or(ParserError::Expected("value@var_declaration <expression>"))?;

            Ok(Some(VarDeclaration { name: name.clone(), value }))
        } else {
            Err(ParserError::Expected("name@var_declaration <identifier>"))
        }
    }
}