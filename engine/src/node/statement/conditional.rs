use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    lang::{parser::{context::ParsingContext, cursor::ignore_eols, error::ParserError}, token::{keyword::TokenLangKeyword, Token}}, node::{ ASTNode, Node, NodeParserTickResult}, vm::tick::VMTick
};

use super::{scope::ScopeStruct, StatementNode, StatementResult};

/// A conditional statement that will run a branch based on a condition
#[derive(Debug, Clone)]
pub struct ConditionalStatement {
    /// The condition that will be checked to decide which branch to run
    pub condition: Box<ASTNode>,

    /// The branch to run if the condition is true
    pub then_branch: ScopeStruct,

    /// The branch to run if the condition is false (or no other condition was met)
    pub else_branch: Option<ScopeStruct>,
}

impl Node for ConditionalStatement {
    /// Transcribe a conditional statement from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Conditionals should start with the keyword `if`
        cursor.expect(&Token::LangKeyword(TokenLangKeyword::If), ParserError::Expected("start@conditional <keyword:if> 'if'".to_string()))?;

        ignore_eols(cursor);

        // Get the condition expression
        let condition = Box::new(ASTNode::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("condition@conditional <node>".to_string())))?);

        ignore_eols(cursor);

        // Get a scope with the branch to run if the condition is true
        let then_branch = ScopeStruct::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("then_branch@conditional <node>".to_string())))?;

        ignore_eols(cursor);

        // Optionally, if the statement continues with the `else` keyword, get the else branch
        let else_branch = if cursor.peek() == Some(&Token::LangKeyword(TokenLangKeyword::Else)) {
            // TODO: Automate this in the cursor?
            cursor.next();
            ignore_eols(cursor);
            Some(ScopeStruct::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("else_branch@conditional <node>".to_string())))?)
        } else {
            // Keep last EOL
            cursor.back();
            None
        };

        Ok(Some(Self { condition, then_branch, else_branch }))
    }
}

impl StatementNode for ConditionalStatement {
    /// Run the conditional statement (with the corresponding branch) and return a value
    fn execute(&self, tick: &mut VMTick) -> Option<StatementResult> {
        if self.condition.evaluate(tick).into() {
            self.then_branch.execute(tick)
        } else if let Some(else_branch) = &self.else_branch {
            else_branch.execute(tick)
        } else {
            None
        }
    }
}

impl fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if {} {}", self.condition, self.then_branch)?;

        if let Some(else_branch) = &self.else_branch {
            write!(f, " else {}", else_branch)?;
        }

        Ok(())
    }
}