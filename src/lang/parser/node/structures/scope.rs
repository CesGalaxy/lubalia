use crate::{lang::{parser::{error::ParserError, node::{ASTNode, Node}}, token::{Token, TokenSymbol}}, utils::transcriber::cursor::TranscriberCursor};

#[derive(Debug, Clone)]
pub struct ScopeStruct {
    #[allow(dead_code)]
    code: Vec<ASTNode>
}

impl Node for ScopeStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ScopeStruct>, ParserError> {
        let mut buffer = vec![];

        // TODO: Integrate this in the cursor
        if !cursor.consume().is_some_and(|t| t == &Token::Symbol(TokenSymbol::BraceOpen)) {
            return Err(ParserError::Expected("start@scope/sym <sym:brace:open> '{'".to_string()));
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
            return Err(ParserError::Expected("end@scope/sym <sym:brace:close> '}'".to_string()));
        }

        Ok(Some(ScopeStruct {
            code: buffer,
        }))
    }
}