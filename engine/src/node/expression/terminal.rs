use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    data::DataValue,
    lang::{parser::{context::ParsingContext, error::{expected_token, ParserError}}, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}},
    node::{ASTNode, Node, NodeParserTickResult},
    vm::tick::VMTick
};

use super::{list::LiteralListExpression, ufunc_constructor::UnnamedFunctionConstructor, ExpressionNode};

/// An expression which evaluated result doesn't need manipulation
#[derive(Debug, Clone)]
pub enum TerminalExpression {
    /// A value provided in the code
    StaticLiteral(DataValue),

    /// A list of values provided in the core
    StaticLiteralList(LiteralListExpression),

    /// A reference to a variable (thorugh its name)
    VarRef(String),

    /// A reference to the last evaluated expression value
    LastValue,

    /// An unnamed function
    UnnamedFunction(UnnamedFunctionConstructor),

    /// Content inside a parenthesis
    Parenthesis(Box<ASTNode>)
}

impl Node for TerminalExpression {
    /// Transcribe a terminal expression (literal, variable reference, scope, etc.)
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        match cursor.consume() {
            Some(Token::Literal(literal)) => Ok(Some(Self::StaticLiteral(literal.clone().into()))),
            Some(Token::LangKeyword(keyword)) => match keyword {
                TokenLangKeyword::True => Ok(Some(Self::StaticLiteral(DataValue::Boolean(true)))),
                TokenLangKeyword::False => Ok(Some(Self::StaticLiteral(DataValue::Boolean(false)))),
                TokenLangKeyword::Null => Ok(Some(Self::StaticLiteral(DataValue::Null))),
                TokenLangKeyword::Fn => {
                    cursor.back();
                    UnnamedFunctionConstructor::transcribe(cursor, ctx).map(|o| o.map(Self::UnnamedFunction))
                },
                _ => Err(TranscriptionException::NotFound(expected_token!(LangKeyword; <expr:terminal>)))
            },
            Some(Token::CustomKeyword(keyword)) => Ok(Some(Self::VarRef(keyword.clone()))),
            Some(Token::Symbol(TokenSymbol::Underscore)) => Ok(Some(Self::LastValue)),
            Some(Token::Symbol(TokenSymbol::ParenOpen)) => {
                // TODO: Does Result have a way of simplifying this?
                let node = ASTNode::transcribe(cursor, ctx).map(|o| o.map(Box::new).map(Self::Parenthesis));
                cursor.expect(&Token::Symbol(TokenSymbol::ParenClose), ParserError::Expected(expected_token!(ParenClose; <expr:terminal>)))?;
                node
            },
            Some(Token::Symbol(TokenSymbol::BracketOpen)) => {
                cursor.back();
                LiteralListExpression::transcribe(cursor, ctx).map(|o| o.map(Self::StaticLiteralList))
            },
            _ => Err(TranscriptionException::NotFound(expected_token!(<expr:terminal>)))
        }
    }
}

impl ExpressionNode for TerminalExpression {
    /// Evaluate the expression and return its value
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        match self {
            Self::StaticLiteral(literal) => literal.clone(),
            Self::StaticLiteralList(list) => list.evaluate(tick),
            Self::VarRef(varname) => tick.get_context().get(varname.clone()).cloned().unwrap_or_default(),
            Self::LastValue => tick.vm.last_value.clone(),
            Self::UnnamedFunction(constructor) => constructor.evaluate(tick),
            Self::Parenthesis(node) => node.evaluate(tick)
        }
    }
}

impl fmt::Display for TerminalExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaticLiteral(literal) => write!(f, "{literal}"),
            Self::StaticLiteralList(list) => write!(f, "{list}"),
            Self::VarRef(varname) => write!(f, "{varname}"),
            Self::LastValue => write!(f, "_"),
            Self::UnnamedFunction(ufn) => write!(f, "{ufn}"),
            Self::Parenthesis(node) => write!(f, "({node})")
        }
    }
}