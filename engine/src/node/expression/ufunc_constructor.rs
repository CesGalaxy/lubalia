use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{data::DataValue, lang::{parser::{cursor::ignore_eols, error::ParserError}, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}}, node::{statement::scope::ScopeStruct, Node, NodeParserTickResult}, vm::tick::VMTick};

use super::ExpressionNode;

#[derive(Debug, Clone)]
pub struct UnnamedFunctionConstructor {
    /// The arguments of the function
    args: Vec<String>,

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

        let mut args = vec![];

        // Get the arguments of the function (for now, no commas are required/allowed)
        while let Some(Token::CustomKeyword(arg)) = cursor.peek() {
            cursor.next();

            args.push(arg.clone());

            ignore_eols(cursor);
        }

        // Then, a closing parenthesis should follow
        if cursor.consume() != Some(&Token::Symbol(TokenSymbol::ParenClose)) {
            return Err(TranscriptionException::Error(ParserError::Expected("args_end@ufn/sym <sym:paren:close> ')'".to_string())));
        }

        ignore_eols(cursor);

        // The body of the function is a scope
        let body = ScopeStruct::transcribe(cursor)?.ok_or(TranscriptionException::Error(ParserError::Expected("body@ufn <scope>".to_string())))?;

        Ok(Some(Self { args, body }))
    }
}

impl ExpressionNode for UnnamedFunctionConstructor {
    fn evaluate(&self, _tick: &mut VMTick) -> DataValue {
        DataValue::Callable(self.args.clone(), self.body.clone())
    }
}

impl fmt::Display for UnnamedFunctionConstructor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn({}) {{\n{}\n}}", self.args.join(", "), self.body)
    }
}