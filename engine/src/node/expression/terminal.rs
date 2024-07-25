use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::{
    data::DataValue,
    node::{statement::{ASTStatement, StatementNode}, Node},
    lang::{parser::error::ParserError, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}},
    vm::VMTick
};

use super::ExpressionNode;

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
    LastValue
}

impl Node for TerminalExpression {
    /// Transcribe a terminal expression (literal, variable reference, scope, etc.)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<TerminalExpression>, ParserError> {
        fn return_statament(cursor: &mut TranscriberCursor<Token>) -> Result<Option<TerminalExpression>, ParserError> {
            cursor.back();
            ASTStatement::transcribe(cursor).map(|o| o.map(|stmt| TerminalExpression::StatementResult(Box::new(stmt))))
        }
        
        match cursor.consume() {
            Some(Token::Literal(literal)) => Ok(Some(Self::Literal(literal.clone().into()))),
            Some(Token::LangKeyword(keyword)) => match keyword {
                TokenLangKeyword::True => Ok(Some(Self::Literal(DataValue::Boolean(true)))),
                TokenLangKeyword::False => Ok(Some(Self::Literal(DataValue::Boolean(false)))),
                TokenLangKeyword::Null => Ok(Some(Self::Literal(DataValue::Null))),
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
            Self::StatementResult(statement) => statement.execute(tick).unwrap_or_default(),
            Self::LastValue => tick.vm.last_value.clone()
        }
    }
}

impl std::fmt::Display for TerminalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::VarRef(varname) => write!(f, "{}", varname),
            Self::StatementResult(stmt) => write!(f, "~{}", stmt),
            Self::LastValue => write!(f, "_")
        }
    }
}