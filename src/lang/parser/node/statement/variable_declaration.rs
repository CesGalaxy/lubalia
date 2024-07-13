use colored::Colorize;

use crate::lang::{
    token::{Token, TokenSymbol},
    parser::{
        exception::{ExpectedToken, ParserError, ParserException},
        machine::ParsingMachine,
        node::{expression::{Expression, ExpressionNode}, Node, NodeFactory}
    }
};

use super::StatementNode;

#[derive(Debug, Clone)]
pub struct VariableDeclarationNode(String, Expression);

impl Node for VariableDeclarationNode {}

impl NodeFactory for VariableDeclarationNode {
    /// Search for: the var keyword, the var name, the '=' symbol, an expression as value and the ';' symbol
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        m.next();

        if let Some(Token::Keyword(varname)) = m.consume() {
            if let Some(Token::Symbol(TokenSymbol::Equal)) = m.consume() {
                let value = Expression::from_tokens(m)?;

                println!("{value}");

                if let Some(Token::Semicolon) = m.consume() {
                    Ok(Self(varname, value))
                } else {
                    Err(m.err(ParserException::TokenExpected(ExpectedToken::Symbol(";"))))
                }
            } else {
                Err(m.err(ParserException::TokenExpected(ExpectedToken::Symbol("="))))
            }
        } else {
            Err(m.err(ParserException::TokenExpected(ExpectedToken::Keyword("<var name>"))))
        }
    }
}

impl StatementNode for VariableDeclarationNode {
    /// Set the variable in the scope with the evaluated value
    fn run(&self, scope: &mut crate::vm::context::Context) {
        scope.set(self.0.clone(), self.1.evaluate(scope))
    }
}

impl std::fmt::Display for VariableDeclarationNode {
    /// Format: [String / var name] = [Expression / var value]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.0.bold(), self.1)
    }
}
