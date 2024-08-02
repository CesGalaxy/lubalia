use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{data::DataValue, lang::{parser::{cursor::ignore_eols, error::ParserError}, token::{symbol::TokenSymbol, Token}}, node::{expression::{ASTExpression, ExpressionNode}, Node, NodeParserTickResult}, vm::{context::Context, tick::VMTick}};

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

        if let DataValue::Callable(required_args, optional_args, body) = called {
            if args.len() < required_args.len() {
                panic!("Param {} is required", required_args[args.len()]);
            }

            let mut variables: Vec<(String, DataValue)> = required_args.iter().cloned().zip(args.clone()).collect();

            let mut i = 0;

            for (name, default) in optional_args {
                variables.push((name, args.get(i).map(|v| v.clone()).unwrap_or(default)));
                i += 1;
            }

            let is_global_context = tick.context.is_none();

            let parent_ctx = tick.get_context().clone();
            tick.context = Some(Box::new(Context::with_parent(variables, Some(parent_ctx))));

            let result = body.execute(tick);

            if let Some(child) = &tick.context {
                tick.context = if is_global_context {
                    tick.vm.global = *child.parent.clone().expect("Matryoshka: global context missing!");
                    None
                } else {
                    child.parent.clone()
                }
            }

            result
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