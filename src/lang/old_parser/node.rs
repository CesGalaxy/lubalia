pub mod expression;
pub mod statement;
pub mod scope;

use crate::lang::token::{Token, TokenSymbol};

use super::{exception::{ParserError, ParserException}, machine::ParsingMachine};

pub trait Node: std::fmt::Debug + std::fmt::Display {}

pub trait NodeFactory: Node {
    /// Parses a token stream into a node or an error.
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> where Self: Sized;
}

#[derive(Debug, Clone)]
pub enum TreeNode {
    Expression(expression::Expression),
    Statement(statement::Statement),
    Scope(scope::ScopeNode)
}

impl Node for TreeNode {}

impl NodeFactory for TreeNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        match m.peek().expect("We want a TOKEN") {
            Token::Keyword(keyword) => match keyword.as_str() {
                // TODO: Support for 'let', 'let var' and 'let const'
                "let" => Ok(Self::Statement(statement::Statement::VariableDeclaration(
                    statement::variable_declaration::VariableDeclarationNode::from_tokens(m)?
                ))),
                // TODO: Continue the match (keyword can be a variable)
                _ => panic!("Invalid keyword"),
            },
            Token::Symbol(TokenSymbol::BracketOpen) => Ok(Self::Scope(scope::ScopeNode::from_tokens(m)?)),
            // If the new node is not an statement, check for an expression (which will be printed when evaluating it).
            // In case that the expression isn't valid neither, an error will be thrown.
            _ => match expression::Expression::from_tokens(m) {
                Ok(expression) => Ok(Self::Expression(expression)),
                Err(error) => Err(m.err(ParserException::InvalidToken(
                    m.peek().unwrap().clone(),
                    Box::new(error)
                ))),
            }
        }
    }
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeNode::Expression(expression) => write!(f, "[ {expression} ]"),
            TreeNode::Statement(statement) => write!(f, "( {statement} )"),
            TreeNode::Scope(scope) => write!(f, "{{\n{scope}}}")
        }
    }
}

pub type AbstractSyntaxTree = Vec<TreeNode>;