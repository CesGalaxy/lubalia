use lubalia_utils::transcriber::cursor::TranscriberCursor;

use crate::{
    data::DataValue,
    node::{expression::{ASTExpression, ExpressionNode}, Node},
    lang::{parser::error::ParserError, token::{keyword::TokenLangKeyword, Token}},
    vm::tick::VMTick
};

use super::{scope::ScopeStruct, StatementNode};

/// A conditional statement that will run a branch based on a condition
#[derive(Debug, Clone)]
pub struct ConditionalStatement {
    /// The condition that will be checked to decide which branch to run
    pub condition: Box<ASTExpression>,

    /// The branch to run if the condition is true
    pub then_branch: ScopeStruct,

    /// The branch to run if the condition is false (or no other condition was met)
    pub else_branch: Option<ScopeStruct>,
}

impl Node for ConditionalStatement {
    /// Transcribe a conditional statement from the source code (tokens)
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized {
        // Conditionals should start with the keyword `if`
        if cursor.consume() != Some(&Token::LangKeyword(TokenLangKeyword::If)) {
            return Err(ParserError::Expected("start@conditional <keyword:if> 'if'".to_string()));
        }

        // Get the condition expression
        let condition = ASTExpression::transcribe(cursor)?.ok_or(ParserError::Expected("condition@conditional <expr>".to_string()))?;

        // Get a scope with the branch to run if the condition is true
        let then_branch = ScopeStruct::transcribe(cursor)?.ok_or(ParserError::Expected("then_branch@conditional <node>".to_string()))?;

        // Optionally, if the statement continues with the `else` keyword, get the else branch
        let else_branch = if cursor.consume() == Some(&Token::LangKeyword(TokenLangKeyword::Else)) {
            Some(ScopeStruct::transcribe(cursor)?.ok_or(ParserError::Expected("else_branch@conditional <node>".to_string()))?)
        } else {
            None
        };

        Ok(Some(Self {
            condition: Box::new(condition),
            then_branch: then_branch,
            else_branch: else_branch
        }))
    }
}

impl StatementNode for ConditionalStatement {
    /// Run the conditional statement (with the corresponding branch) and return a value
    fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        // [SemiExpression] Return the value of the new variable
        if self.condition.evaluate(tick).into() {
            self.then_branch.execute(tick)
        } else if let Some(else_branch) = &self.else_branch {
            else_branch.execute(tick)
        } else {
            None
        }
    }
}

impl std::fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.condition, self.then_branch)?;

        if let Some(else_branch) = &self.else_branch {
            write!(f, " else {}", else_branch)?;
        }

        Ok(())
    }
}