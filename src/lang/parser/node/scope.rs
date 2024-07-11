use crate::lang::{
    lexer::token::{Token, TokenSymbol},
    parser::{
        exception::{ExpectedToken, ParserError, ParserException},
        machine::ParsingMachine
    }
};

use super::{ AbstractSyntaxTree, Node, NodeFactory, TreeNode};

#[derive(Debug, Clone)]
pub struct ScopeNode(AbstractSyntaxTree);

impl Node for ScopeNode {}

impl NodeFactory for ScopeNode {
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        // Check if the scope is initialized with a '{'
        // Don't do the check if it's the strat of the code
        if m.pos != 0 && m.consume() != Some(Token::Symbol(TokenSymbol::BracketOpen)) {
            return Err(m.except(ParserException::TokenExpected(ExpectedToken::Symbol("<scope init>"))));
        }

        let mut tree: AbstractSyntaxTree = Vec::new();

        while let Some(t) = m.peek() {
            match t {
                Token::Symbol(TokenSymbol::BracketClose) => {
                    m.next();
                    return Ok(Self(tree));
                },
                Token::EOL => { m.next(); },
                Token::EOF => return Ok(Self(tree)),
                _ => tree.push(TreeNode::from_tokens(m)?),
            }
        }

        return Err(m.except(ParserException::TokenExpected(ExpectedToken::Symbol("}"))));
    }
}

impl std::fmt::Display for ScopeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.0 {
            write!(f, "{}\n", node)?;
        }
        Ok(())
    }
}