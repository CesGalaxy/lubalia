use lubalia_utils::transcriber::{cursor::TranscriberCursor, error::TranscriptionException};

use crate::lang::{
    parser::{context::ParsingContext, error::{expected_token, ParserError}},
    syntax::node::{expression::{terminal::TerminalExpression, ASTExpression}, Node, NodeParserTickResult},
    token::Token
};

use super::{operator::BinaryOperator, BinaryExpression};

impl Node for BinaryExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        // Use terminal expr due to the infinite loop generated when transcribing a normal expression
        // TODO: Search first for statement and then terminal? Or use parenthesis (terminal)?
        let lhs = Box::new(ASTExpression::Terminal(TerminalExpression::transcribe(cursor, ctx)?
            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(lhs@bi <expr>))))?));

        let operator = BinaryOperator::transcribe(cursor)?
            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(operator@bi_op | operator@bi <operator>))))?;

        let rhs = Box::new(ASTExpression::Terminal(TerminalExpression::transcribe(cursor, ctx)?
            .ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(rhs@bi <expr>))))?));

        Ok(Some(Self { lhs, operator, rhs }))
    }
}