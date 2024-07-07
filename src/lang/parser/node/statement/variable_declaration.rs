use crate::lang::{lexer::token::{Token, TokenSymbol}, parser::{exception::{ExcpectedToken, ParsingMachineError, ParsingMachineException}, machine::ParsingMachine, node::{expression::Expression, Node, NodeFactory}}};

use super::StatementNode;

#[derive(Debug)]
pub struct VariableDeclarationNode(String, Expression);

impl NodeFactory for VariableDeclarationNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParsingMachineError> {
        m.next();

        if let Some(Token::Keyword(varname)) = m.consume() {
            if let Some(Token::Symbol(TokenSymbol::Equal)) = m.consume() {
                if let Ok(value) = Expression::from_tokens(m) {
                    if let Some(Token::Semicolon) = m.consume() {
                        Ok(Self(varname, value))
                    } else {
                        Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Symbol(";"))))
                    }
                } else {
                    Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Literal("<expr>"))))
                }
            } else {
                Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Symbol("="))))
            }
        } else {
            Err(m.except(ParsingMachineException::TokenExpected(ExcpectedToken::Keyword("<var name>"))))
        }
    }
}

impl StatementNode for VariableDeclarationNode {}

impl Node for VariableDeclarationNode {}