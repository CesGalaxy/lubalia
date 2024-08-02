use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{data::DataValue, lang::{parser::{cursor::ignore_eols, error::ParserError}, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}}, node::{statement::scope::ScopeStruct, Node, NodeParserTickResult}, vm::tick::VMTick};

use super::{ASTExpression, ExpressionNode};

#[derive(Debug, Clone)]
pub struct UnnamedFunctionConstructor {
    /// The arguments of the function
    required_args: Vec<String>,

    /// The optional arguments of the function (can have default values)
    optional_args: Vec<(String, Option<ASTExpression>)>,

    /// The body of the function
    body: ScopeStruct
}

impl Node for UnnamedFunctionConstructor {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> where Self: Sized {
        // Unnamed functions should start with the keyword `fn`
        if cursor.consume() != Some(&Token::LangKeyword(TokenLangKeyword::Fn)) {
            return Err(TranscriptionException::Error(ParserError::Expected("start@ufn <keyword:fn> 'fn'".to_string())));
        }

        // Then, an opening parenthesis should follow
        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::ParenOpen)) {
            return Err(TranscriptionException::Error(ParserError::Expected("args_start@ufn/sym <sym:paren:open> '('".to_string())));
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

                let default_value = ASTExpression::transcribe(cursor)?.ok_or(TranscriptionException::Error(ParserError::Expected("default@ufn <expr>".to_string())))?;

                optional_args.push((arg.clone(), Some(default_value)));
            } else if let Some(Token::Symbol(TokenSymbol::Question)) = cursor.peek() {
                cursor.next();
                ignore_eols(cursor);

                optional_args.push((arg.clone(), None));

            } else {
                required_args.push(arg.clone());
            }

            ignore_eols(cursor);
        }

        // Then, a closing parenthesis should follow
        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::ParenClose)) {
            return Err(TranscriptionException::Error(ParserError::Expected("args_end@ufn/sym <sym:paren:close> ')'".to_string())));
        }

        ignore_eols(cursor);

        // The body of the function is a scope
        let body = ScopeStruct::transcribe(cursor)?.ok_or(TranscriptionException::Error(ParserError::Expected("body@ufn <scope>".to_string())))?;

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