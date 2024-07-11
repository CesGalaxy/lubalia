pub mod node;
mod machine;
mod exception;
pub mod data;
mod arithmetic;

use exception::ParserError;
use machine::ParsingMachine;
use node::{scope::ScopeNode, AbstractSyntaxTree, NodeFactory};

use super::lexer::token::Token;

/// Generates an abstract syntax tree (AST) from a vector of tokens.
pub fn parse_tree(tokens: Vec<Token>) -> Result<AbstractSyntaxTree, ParserError> {
    // let mut tree = Vec::new();

    let mut machine = ParsingMachine::new(tokens);

    // For each new root-node: it will start parsing at its first token,
    // and end the parsing at the first token of the next root-node.
    // while !machine.is_overflow() {
    //     tree.push(TreeNode::from_tokens(&mut machine)?);

    //     // match t {
    //     //     Token::Keyword(keyword) => match keyword.as_str() {
    //     //         "let" => tree.push(
    //     //             TreeNode::Statement(Statement::VariableDeclaration(
    //     //                 statement::variable_declaration::VariableDeclarationNode::from_tokens(&mut machine)?
    //     //             ))
    //     //         ),
    //     //         _ => panic!("Invalid keyword"),
    //     //     },
    //     //     Token::EOL => { machine.next(); },
    //     //     Token::EOF => break,
    //     //     // If the new root-node is not an statement, check for an expression (which will be printed when evaluating it).
    //     //     // In case that the expression isn't valid neither, an error will be thrown.
    //     //     _ => match Expression::from_tokens(&mut machine) {
    //     //         Ok(expression) => tree.push(TreeNode::Expression(expression)),
    //     //         Err(error) => return Err(machine.except(ParserException::InvalidToken(
    //     //             machine.peek().unwrap().clone(),
    //     //             Box::new(error)
    //     //         ))),
    //     //     }
    //     // }
    // }

    Ok(ScopeNode::from_tokens(&mut machine)?.0)
}