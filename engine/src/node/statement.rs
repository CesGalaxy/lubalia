pub mod variable_declaration;
pub mod scope;
pub mod conditional;

use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::cursor::TranscriberCursor};

use crate::{
    data::DataValue,
    lang::{parser::error::ParserError, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}},
    vm::tick::VMTick
};

use super::Node;

/// An instruction the VM executes without returning a value
#[derive(Debug, Clone)]
pub enum ASTStatement {
    VariableDeclaration(variable_declaration::VariableDeclaration),
    Scope(scope::ScopeStruct),
    Conditional(conditional::ConditionalStatement)
}

pub trait StatementNode: Node {
    fn execute(&self, tick: &mut VMTick) -> Option<DataValue>;
}

impl Node for ASTStatement {
    /// Transcribe an statement (if possible)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTStatement>, ParserError> {
        //? Should this return Err if no statement is found? So node transcription ignores all errors and tries an expr (which will the one that can fail)
        //* This must make sure that the transcribed node is the correct one. In case of error, it will fail.
        match cursor.peek() {
            // Statements are usually defined with an initial keyword
            Some(Token::LangKeyword(keyword)) => match keyword {
                TokenLangKeyword::Let => variable_declaration::VariableDeclaration::transcribe(cursor).map(|vd| vd.map(ASTStatement::VariableDeclaration)),
                TokenLangKeyword::If => conditional::ConditionalStatement::transcribe(cursor).map(|cond| cond.map(ASTStatement::Conditional)),
                _ => Err(ParserError::Expected("LangKeyword $ <stmnt>".to_string()))
            },
            // Scopes are statements too
            Some(Token::Symbol(TokenSymbol::BraceOpen)) => scope::ScopeStruct::transcribe(cursor).map(|scope| scope.map(Self::Scope)),
            _ => Err(ParserError::Expected("<stmnt>".to_string()))
        }
    }
}

impl StatementNode for ASTStatement {
    /// Execute an statement and return a value if any is provided
    fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        match self {
            ASTStatement::VariableDeclaration(vd) => vd.execute(tick),
            ASTStatement::Scope(scope) => scope.execute(tick),
            ASTStatement::Conditional(cond) => cond.execute(tick)
        }
    }
}

impl fmt::Display for ASTStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTStatement::VariableDeclaration(vd) => write!(f, "{}", vd),
            ASTStatement::Scope(scope) => write!(f, "{}", scope),
            ASTStatement::Conditional(cond) => write!(f, "{}", cond)
        }
    }
}