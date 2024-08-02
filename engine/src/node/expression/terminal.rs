use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::{
    data::DataValue,
    lang::token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token},
    node::{statement::ASTStatement, Node, NodeParserTickResult},
    vm::tick::VMTick
};

use super::{ufunc_constructor::UnnamedFunctionConstructor, ExpressionNode};

/// An expression which evaluated result doesn't need manipulation
#[derive(Debug, Clone)]
pub enum TerminalExpression {
    /// A value provided in the code
    Literal(DataValue),

    /// A reference to a variable (thorugh its name)
    VarRef(String),

    /// A scope (a block of code)
    StatementResult(Box<ASTStatement>),

    /// A reference to the last evaluated expression value
    LastValue,

    /// An unnamed function
    UnnamedFunction(UnnamedFunctionConstructor),

    // A call to a function
    //FunctionCall(String, Vec<TerminalExpression>)
}

impl Node for TerminalExpression {
    /// Transcribe a terminal expression (literal, variable reference, scope, etc.)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> {
        fn return_statament(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<TerminalExpression> {
            cursor.back();
            ASTStatement::transcribe(cursor).map(|o| o.map(|stmt| TerminalExpression::StatementResult(Box::new(stmt))))
        }

        match cursor.consume() {
            Some(Token::Literal(literal)) => Ok(Some(Self::Literal(literal.clone().into()))),
            Some(Token::LangKeyword(keyword)) => match keyword {
                TokenLangKeyword::True => Ok(Some(Self::Literal(DataValue::Boolean(true)))),
                TokenLangKeyword::False => Ok(Some(Self::Literal(DataValue::Boolean(false)))),
                TokenLangKeyword::Null => Ok(Some(Self::Literal(DataValue::Null))),
                TokenLangKeyword::Fn => {
                    cursor.back();
                    UnnamedFunctionConstructor::transcribe(cursor).map(|o| o.map(Self::UnnamedFunction))
                },
                _ => return_statament(cursor)
            },
            Some(Token::CustomKeyword(keyword)) => Ok(Some(Self::VarRef(keyword.clone()))),
            Some(Token::Symbol(TokenSymbol::Underscore)) => Ok(Some(Self::LastValue)),
            _ => return_statament(cursor)
        }
    }
}

impl ExpressionNode for TerminalExpression {
    /// Evaluate the expression and return its value
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::VarRef(varname) => tick.get_context().get(varname.clone()).cloned().unwrap_or_default(),
            Self::StatementResult(statement) => statement.evaluate(tick),
            Self::LastValue => tick.vm.last_value.clone(),
            Self::UnnamedFunction(constructor) => constructor.evaluate(tick)
        }
    }
}

impl fmt::Display for TerminalExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::VarRef(varname) => write!(f, "{varname}"),
            Self::StatementResult(stmnt) => write!(f, "~{stmnt}"),
            Self::LastValue => write!(f, "_"),
            Self::UnnamedFunction(ufn) => write!(f, "{ufn}")
        }
    }
}