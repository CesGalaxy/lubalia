use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::lang::{parser::{context::ParsingContext, error::ParserError}, syntax::node::{statement::ASTStatement, ASTNode, Node, NodeParserTickResult}, token::{symbol::TokenSymbol, Token}};

use super::{meta::BlockMetadata, BlockStruct};

impl Node for BlockStruct {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Blocks should start with an opening brace
        cursor.expect(&Token::Symbol(TokenSymbol::BraceOpen), ParserError::Expected("start@scope/sym <sym:brace:open> '{'".to_string()))?;

        let mut nodes = vec![];
        let mut meta = BlockMetadata::default();

        // Save all nodes found inside the scope until a closing brace is found (and ends the scope)
        // FIXME: When None it will loop forever
        while Some(&Token::Symbol(TokenSymbol::BraceClose)) != cursor.peek() {
            let initial_position = cursor.pos;

            if let Some(node) = ASTNode::transcribe(cursor, ctx)? {
                match node {
                    ASTNode::Statement(ASTStatement::VariableDeclaration(_)) => {
                        meta.variables += 1;
                    },
                    _ => {}
                }

                nodes.push(node);
            }

            // FIXME: This is a bad idea
            if cursor.pos == initial_position {
                cursor.next();
            }
        }

        // Blocks should end with a closing brace
        cursor.expect(&Token::Symbol(TokenSymbol::BraceClose), ParserError::Expected("end@scope/sym <sym:brace:close> '}'".to_string()))?;

        Ok(Some(BlockStruct { nodes, name: String::new(), meta }))
    }
}