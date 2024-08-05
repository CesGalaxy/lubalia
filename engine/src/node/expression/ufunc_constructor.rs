use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    data::DataValue, lang::{parser::{context::ParsingContext, cursor::ignore_eols, error::{expected_token, ParserError}}, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}}, node::{ASTNode, Node, NodeParserTickResult}, vm::tick::VMTick};

use super::{ASTExpression, ExpressionNode};

#[derive(Debug, Clone)]
pub struct UnnamedFunctionConstructor {
    /// The arguments of the function
    required_args: Vec<String>,

    /// The optional arguments of the function (can have default values)
    optional_args: Vec<(String, Option<ASTExpression>)>,

    /// The body of the function
    body: Box<ASTNode>,
}

impl Node for UnnamedFunctionConstructor {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Unnamed functions should start with the keyword `fn`
        if cursor.consume() != Some(&Token::LangKeyword(TokenLangKeyword::Fn)) {
            return Err(TranscriptionException::Error(ParserError::Expected(expected_token!(start@ufn <keyword:fn>))));
        }

        // Then, an opening parenthesis should follow
        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::ParenOpen)) {
            return Err(TranscriptionException::Error(ParserError::Expected(expected_token!(args_start@ufn <sym:paren:open>))));
        }

        ignore_eols(cursor);

        let mut required_args = vec![];
        let mut optional_args = vec![];

        // Get the arguments of the function (for now, no commas are required/allowed)
        while let Some(Token::CustomKeyword(arg)) = cursor.peek() {
            cursor.next();
            ignore_eols(cursor);

            if let Some(Token::Symbol(TokenSymbol::Equal)) = cursor.peek() {
                cursor.next();
                ignore_eols(cursor);

                let default_value = ASTExpression::transcribe(cursor, ctx)?
                    .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(default@ufn <expr>))))?;

                optional_args.push((arg.clone(), Some(default_value)));
            } else if let Some(Token::Symbol(TokenSymbol::Question)) = cursor.peek() {
                cursor.next();
                optional_args.push((arg.clone(), None));

            } else {
                required_args.push(arg.clone());
            }

            ignore_eols(cursor);

            if let Some(Token::Symbol(TokenSymbol::Comma)) = cursor.peek() {
                cursor.next();
                ignore_eols(cursor);
            }
        }

        // Then, a closing parenthesis should follow
        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::ParenClose)) {
            return Err(TranscriptionException::Error(ParserError::Expected(expected_token!(args_end@ufn <sym:paren:close>))));
        }

        ignore_eols(cursor);

        // The body of the function is a scope
        let body = Box::new(ASTNode::transcribe(cursor, ctx)?
            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(body@ufn <node>))))?);

        Ok(Some(Self { required_args, optional_args, body }))
    }
}

impl ExpressionNode for UnnamedFunctionConstructor {
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        let optional_args: Vec<(String, DataValue)> = self.optional_args
            .iter()
            .map(|(name, default)| (name.clone(), default.as_ref().map(|expr| expr.evaluate(tick)).unwrap_or_default()))
            .collect();

        DataValue::Callable(self.required_args.clone(), optional_args.clone(), self.body.clone())
    }
}

impl fmt::Display for UnnamedFunctionConstructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn({}| {}) {{\n{}\n}}",
            self.required_args.join(", "),
            self.optional_args.iter().map(|(name, default)| {
                match default {
                    Some(default) => format!("{} = {}", name, default),
                    None => name.clone()
                }
            }).collect::<Vec<String>>().join(", "),
            self.body
        )
    }
}