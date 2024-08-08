use std::{cell::RefCell, fmt};

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    data::DataValue,
    lang::{
        parser::{context::ParsingContext, cursor::ignore_eols, error::ParserError},
        syntax::node::{expression::{ASTExpression, ExpressionNode}, Node, NodeParserTickResult},
        token::{symbol::TokenSymbol, Token}
    },
    vm::{scope::Scope, VM}
};

use super::{StatementNode, StatementResult};

// TODO: Should this be an statement?
#[derive(Debug, Clone)]
pub struct FunctionCallStatement {
    called: ASTExpression,

    args: Vec<ASTExpression>
}

impl Node for FunctionCallStatement {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        if let Ok(Some(called)) = ASTExpression::transcribe(cursor, ctx) {
            cursor.expect(&Token::Symbol(TokenSymbol::ParenOpen), ParserError::Expected("args_start@call/sym <sym:paren:open> '('".to_string()))?;

            ignore_eols(cursor);

            let mut args = vec![];

            while let Ok(Some(arg)) = ASTExpression::transcribe(cursor, ctx) {
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
    fn execute(&self, vm: &mut VM, scope: &RefCell<Scope>) -> Option<StatementResult> {
        let called = self.called.evaluate(vm, scope);

        let mut args = vec![];

        for arg in &self.args {
            args.push(arg.evaluate(vm, scope));
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

            let child = Scope::with_parent(scope.borrow());

            let child = RefCell::new(child);

            body.execute(vm, &child)
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