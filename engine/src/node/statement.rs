pub mod variable_declaration;
pub mod scope;
pub mod conditional;

use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::{
    data::DataValue,
    lang::{parser::error::ParserError, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}},
    vm::VMTick
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
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<ASTStatement>, ParserError> {
        //? Should this return Err if no statement is found? So node transcription ignores all errors and tries an expr (which will the one that can fail)
        //* This must make sure that the transcribed node is the correct one. In case of error, it will fail.
        match cursor.peek() {
            Some(Token::LangKeyword(keyword)) => match keyword {
                TokenLangKeyword::Let => variable_declaration::VariableDeclaration::transcribe(cursor).map(|vd| vd.map(ASTStatement::VariableDeclaration)),
                TokenLangKeyword::If => conditional::ConditionalStatement::transcribe(cursor).map(|cond| cond.map(ASTStatement::Conditional)),
                _ => Ok(None)
            },
            Some(Token::Symbol(TokenSymbol::BraceOpen)) => scope::ScopeStruct::transcribe(cursor).map(|scope| scope.map(Self::Scope)),
            _ => Ok(None)
        }
    }
}

impl StatementNode for ASTStatement {
    fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        match self {
            ASTStatement::VariableDeclaration(vd) => vd.execute(tick),
            ASTStatement::Scope(scope) => scope.execute(tick),
            ASTStatement::Conditional(cond) => cond.execute(tick)
        }
    }
}

impl std::fmt::Display for ASTStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTStatement::VariableDeclaration(vd) => write!(f, "{}", vd),
            ASTStatement::Scope(scope) => write!(f, "{}", scope),
            ASTStatement::Conditional(cond) => write!(f, "{}", cond)
        }
    }
}