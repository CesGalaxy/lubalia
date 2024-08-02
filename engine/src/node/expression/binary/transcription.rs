use lubalia_utils::transcriber::{cursor::TranscriberCursor, error::TranscriptionException};

use crate::{
    lang::{parser::error::{expected_token, ParserError}, token::Token},
    node::{expression::{terminal::TerminalExpression, ASTExpression}, Node, NodeParserTickResult}
};

use super::{operator::BinaryOperator, BinaryExpression};

impl Node for BinaryExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> {
        let lhs = Box::new(ASTExpression::Terminal(TerminalExpression::transcribe(cursor)?
            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(lhs@bi <expr>))))?));

        let operator = BinaryOperator::transcribe(cursor)?
            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(operator@bi_op | operator@bi <operator>))))?;

        let rhs = Box::new(ASTExpression::Terminal(TerminalExpression::transcribe(cursor)?
            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(rhs@bi <expr>))))?));

        Ok(Some(Self { lhs, operator, rhs }))
    }
}