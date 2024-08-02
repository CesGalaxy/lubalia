use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{data::DataValue, lang::{parser::{cursor::ignore_eols, error::ParserError}, token::{symbol::TokenSymbol, Token}}, node::{expression::{ASTExpression, ExpressionNode}, Node, NodeParserTickResult}, vm::tick::VMTick};

use super::{StatementNode, StatementResult};

#[derive(Debug, Clone)]
pub struct FunctionCallStatement {
    called: ASTExpression,

    args: Vec<ASTExpression>
}

impl Node for FunctionCallStatement {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> where Self: Sized {
        if let Ok(Some(called)) = ASTExpression::transcribe(cursor) {
            cursor.expect(&Token::Symbol(TokenSymbol::ParenOpen), ParserError::Expected("args_start@call/sym <sym:paren:open> '('".to_string()))?;

            ignore_eols(cursor);

            let mut args = vec![];

            while let Ok(Some(arg)) = ASTExpression::transcribe(cursor) {
                args.push(arg);

                ignore_eols(cursor);

                if cursor.peek() == Some(&Token::Symbol(TokenSymbol::ParenClose)) {
                    cursor.next();
                    break;
                }

                cursor.expect(&Token::Symbol(TokenSymbol::Comma), ParserError::Expected("arg_sep@call/sym <sym:comma> ','".to_string()))?;
            }

            Ok(Some(Self { called, args }))
        } else {
            Err(TranscriptionException::Error(ParserError::Expected("start@func_call <keyword:_>".to_string())))
        }
    }
}

impl StatementNode for FunctionCallStatement {
    fn execute(&self, tick: &mut VMTick) -> Option<StatementResult> {
        let called = self.called.evaluate(tick);

        let mut args = vec![];

        for arg in &self.args {
            args.push(arg.evaluate(tick));
        }

        if let DataValue::Callable(_argnames, body) = called {
            body.execute(tick)
        } else {
            // TODO: Please, fix this
            panic!("Function call to non-function value: {}", called);
            //Some(StatementResult::Usable(called))
        }
    }
}

impl fmt::Display for FunctionCallStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function call to {} with arguments: {:?}", self.called, self.args)
    }
}