pub mod node;
mod machine;
mod exception;
pub mod data;

use exception::ParsingMachineError;
use machine::ParsingMachine;
use node::{expression::Expression, statement::{self, Statement}, AbstractSyntaxTree, NodeFactory, TreeNode};

use super::lexer::token::Token;

pub fn parse_tree(tokens: Vec<Token>) -> Result<AbstractSyntaxTree, ParserError> {
    let mut tree = Vec::new();

    let mut machine = ParsingMachine::new(tokens);

    while let Some(t) = machine.peek() {
        // STARTS AT: node[0] - ENDS AT: next_node[0]
        match t {
            Token::Keyword(keyword) => match keyword.as_str() {
                "let" => tree.push(
                    TreeNode::Statement(Statement::VariableDeclaration(
                        statement::variable_declaration::VariableDeclarationNode::from_tokens(&mut machine)?
                    ))
                ),
                _ => panic!("Invalid keyword"),
            },
            Token::EOL => { machine.next(); },
            Token::EOF => break,
            _ => match Expression::from_tokens(&mut machine) {
                Ok(expression) => tree.push(TreeNode::Expression(expression)),
                Err(error) => panic!("Invalid token {:?} - {:?}", machine.peek(), error),
            }
        }
    }

    Ok(tree)
}

#[derive(Debug)]
pub enum ParserError {
    ParsingMachineError(ParsingMachineError)
}
