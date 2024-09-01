use std::cell::RefCell;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    data::DataValue,
    lang::{
        parser::{context::{ParsingContext, ParsingIntent}, cursor::ignore_eols, error::{expected_token, ParserError}},
        syntax::node::{ASTNode, Node, NodeParserTickResult},
        token::{symbol::TokenSymbol, Token}
    },
    vm::{scope::Scope, VM}
};

use super::ExpressionNode;

#[derive(Debug, Clone)]
pub enum LiteralArray {
    /// Make the array from a given item repeated a given number of times
    Repeat(Box<ASTNode>, Box<ASTNode>),

    /// Make the array from a given range of numbers
    Range(Box<ASTNode>, Box<ASTNode>),

    /// A normal list
    List(Vec<ASTNode>),

    /// An empty list
    Empty
}

impl Node for LiteralArray {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // TODO: This function runs twice?
        cursor.expect(&Token::Symbol(TokenSymbol::BracketOpen), ParserError::Expected(expected_token!(<sym:bracket:open>)))?;

        ignore_eols(cursor);

        let list = if let ParsingIntent(Ok(Some(first))) = ctx.intent(cursor, ASTNode::transcribe) {
            match cursor.peek() {
                Some(Token::Symbol(TokenSymbol::Semicolon)) => {
                    cursor.next();

                    let second = ASTNode::transcribe(cursor, ctx)?
                        .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(times@arr_repeat <node>))))?;

                    Self::Repeat(Box::new(first), Box::new(second))
                },
                Some(Token::Symbol(TokenSymbol::Point)) if cursor.peek_next() == Some(&Token::Symbol(TokenSymbol::Point)) => {
                    cursor.next();
                    cursor.next();

                    let end = ASTNode::transcribe(cursor, ctx)?
                        .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(end@arr_range <node>))))?;

                    Self::Range(Box::new(first), Box::new(end))
                },
                _ => {
                    let mut items = vec![first];

                    // TODO: Allow trailing comma
                    while let Some(Token::Symbol(TokenSymbol::Comma)) = cursor.peek() {
                        cursor.next();

                        let item = ASTNode::transcribe(cursor, ctx)?
                            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(item@arr <node>))))?;

                        items.push(item);
                    }

                    Self::List(items)
                }
            }
        } else {
            Self::Empty
        };

        ignore_eols(cursor);

        cursor.expect(&Token::Symbol(TokenSymbol::BracketClose), ParserError::Expected(expected_token!(<sym:bracket:closed>)))?;

        Ok(Some(list))
    }
}

impl ExpressionNode for LiteralArray {
    fn evaluate(&self, vm: &mut VM, scope: &RefCell<Scope>) -> DataValue {
        match self {
            LiteralArray::Repeat(item, times) => {
                let item = item.evaluate(vm, scope);
                let times = times.evaluate(vm, scope);

                let mut list = Vec::new();

                for _ in 0..usize::from(times) {
                    list.push(item.clone());
                }

                DataValue::List(list)
            },
            LiteralArray::Range(start, end) => {
                let start = usize::from(start.evaluate(vm, scope));
                let end = usize::from(end.evaluate(vm, scope));

                let mut list = Vec::new();

                for i in start..end {
                    list.push(DataValue::Number(i as f64));
                }

                DataValue::List(list)
            },
            LiteralArray::List(items) => {
                let mut list = Vec::new();

                for item in items {
                    list.push(item.evaluate(vm, scope));
                }

                DataValue::List(list)
            },
            LiteralArray::Empty => DataValue::List(Vec::new())
        }
    }
}

impl std::fmt::Display for LiteralArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralArray::Repeat(item, times) => write!(f, "[Repeat {times} times: {item}]"),
            LiteralArray::Range(start, end) => write!(f, "[Range from {start} until {end}]"),
            LiteralArray::List(items) => {
                let mut list_str = String::new();

                for item in items {
                    list_str.push_str(&format!("{item}, "));
                }

                // Remove the last comma and space
                if !list_str.is_empty() {
                    list_str.pop();
                    list_str.pop();
                }

                write!(f, "[ {} ]", list_str)
            },
            LiteralArray::Empty => write!(f, "[]")
        }
    }
}