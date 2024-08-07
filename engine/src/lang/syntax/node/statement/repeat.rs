use std::fmt;

use lubalia_utils::transcriber::{cursor::TranscriberCursor, error::TranscriptionException};

use crate::lang::{
    parser::{context::ParsingContext, cursor::ignore_eols, error::ParserError},
    syntax::node::{expression::ExpressionNode, ASTNode, Node, NodeParserTickResult},
    token::{keyword::TokenLangKeyword, Token}
};

use super::{ASTStatement, StatementNode, StatementResult};

#[derive(Debug, Clone)]
pub struct Repeat {
    /// The number of times the loop will repeat
    times: Box<ASTNode>,

    /// What will be executed on each iteration of the loop
    iteration: Box<ASTStatement>
}

impl Node for Repeat {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Repeat loops should start with the keyword `repeat`
        cursor.expect(&Token::LangKeyword(TokenLangKeyword::Repeat), ParserError::Expected("start@repeat <keyword:repeat> 'repeat'".to_string()))?;

        ignore_eols(cursor);

        let times = Box::new(ASTNode::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("times@repeat <node>".to_string())))?);

        ignore_eols(cursor);

        let iteration = Box::new(ASTStatement::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("iteration@repeat <stmt>".to_string())))?);

        Ok(Some(Self { times, iteration }))
    }
}

impl StatementNode for Repeat {
    fn execute(&self, tick: &mut crate::vm::tick::VMTick) -> Option<StatementResult> {
        let times: usize = self.times.evaluate(tick).into();

        // TODO: Provide current count to the iteration
        for _ in 0..times {
            if let Some(StatementResult::Return(value)) = self.iteration.execute(tick) {
                return Some(StatementResult::Return(value));
            }
        }

        None
    }
}

impl fmt::Display for Repeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Repeat {} times: {}", self.times, self.iteration)
    }
}