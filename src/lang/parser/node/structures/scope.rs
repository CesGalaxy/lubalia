use crate::{lang::{parser::{error::ParserError, node::{ASTNode, Node}}, token::{Token, TokenSymbol}}, utils::transcriber::cursor::TranscriberCursor};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScopeStruct {
    code: Vec<ASTNode>
}

impl Node for ScopeStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ScopeStruct>, ParserError> {
        let mut buffer = vec![];

        println!("{:?}", cursor.peek());

        if !cursor.consume().is_some_and(|t| t == &Token::Symbol(TokenSymbol::BraceOpen)) {
            return Err(ParserError::Expected("start@scope/sym '{'".to_string()));
        }

        while Some(&Token::Symbol(TokenSymbol::BraceClose)) != cursor.peek() {
            let initial_position = cursor.pos;

            if let Some(node) = ASTNode::transcribe(cursor)? {
                buffer.push(node);
            }

            // FIXME: This is a bad idea
            if cursor.pos == initial_position {
                cursor.next();
            }
        }

        if !cursor.consume().is_some_and(|t| t == &Token::Symbol(TokenSymbol::BraceClose)) {
            return Err(ParserError::Expected("end@scope/sym '}'".to_string()));
        }

        Ok(Some(ScopeStruct {
            code: buffer,
        }))
    }
}