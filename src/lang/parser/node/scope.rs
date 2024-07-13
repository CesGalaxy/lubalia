use crate::lang::{
    token::{Token, TokenSymbol},
    parser::{
        exception::{ExpectedToken, ParserError, ParserException},
        machine::ParsingMachine
    }
};

use super::{AbstractSyntaxTree, Node, NodeFactory, TreeNode};

#[derive(Debug, Clone)]
pub struct ScopeNode(pub AbstractSyntaxTree);

impl Node for ScopeNode {}

impl NodeFactory for ScopeNode {
    /// Try to get an AST (vec of nodes) from a vec of tokens
    fn from_tokens(m: &mut ParsingMachine) -> Result<Self, ParserError> {
        // Check if the scope is initialized with a '{'
        // Don't do the check if it's the strat of the code
        if m.pos != 0 && m.consume() != Some(Token::Symbol(TokenSymbol::BracketOpen)) {
            return Err(m.err(ParserException::TokenExpected(ExpectedToken::Symbol("<scope init>"))));
        }

        let mut tree: AbstractSyntaxTree = Vec::new();

        // For each new root-node: it will start parsing at its first token,
        // and end the parsing at the first token of the next root-node.
        while let Some(t) = m.peek() {
            match t {
                // The closed bracket '}' will end the scope
                // TODO: Don't allow this in the global scope
                Token::Symbol(TokenSymbol::BracketClose) => {
                    m.next();
                    return Ok(Self(tree));
                },
                // Handle end of line/file
                Token::EOL => { m.next(); },
                Token::EOF => return Ok(Self(tree)),
                // Try to parse the current node
                _ => tree.push(TreeNode::from_tokens(m)?),
            }
        }

        // If the scope wasn't closed by a '}' or EOF, then it's missing a '}'
        return Err(m.err(ParserException::TokenExpected(ExpectedToken::Symbol("}"))));
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

impl From<ScopeNode> for AbstractSyntaxTree {
    fn from(node: ScopeNode) -> Self {
        node.0
    }
}

impl From<AbstractSyntaxTree> for ScopeNode {
    fn from(tree: AbstractSyntaxTree) -> Self {
        ScopeNode(tree)
    }
}