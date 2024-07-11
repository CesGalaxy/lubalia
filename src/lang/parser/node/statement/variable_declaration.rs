use colored::Colorize;

use crate::lang::{
    lexer::token::{Token, TokenSymbol},
    parser::{
        exception::{ExcpectedToken, ParserError, ParserException},
        machine::ParsingMachine,
        node::{expression::{Expression, ExpressionNode}, Node, NodeFactory}
    }
};

use super::StatementNode;

#[derive(Debug, Clone)]
pub struct VariableDeclarationNode(String, Expression);

impl Node for VariableDeclarationNode {}

impl NodeFactory for VariableDeclarationNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        m.next();

        if let Some(Token::Keyword(varname)) = m.consume() {
            if let Some(Token::Symbol(TokenSymbol::Equal)) = m.consume() {
                let value = Expression::from_tokens(m)?;

                println!("{value}");

                if let Some(Token::Semicolon) = m.consume() {
                    Ok(Self(varname, value))
                } else {
                    Err(m.except(ParserException::TokenExpected(ExcpectedToken::Symbol(";"))))
                }
            } else {
                Err(m.except(ParserException::TokenExpected(ExcpectedToken::Symbol("="))))
            }
        } else {
            Err(m.except(ParserException::TokenExpected(ExcpectedToken::Keyword("<var name>"))))
        }
    }
}

impl StatementNode for VariableDeclarationNode {
    fn run(&self, scope: &mut crate::vm::context::Context) {
        scope.set(self.0.clone(), self.1.evaluate(scope))
    }
}

impl std::fmt::Display for VariableDeclarationNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.0.bold(), self.1)
    }
}
