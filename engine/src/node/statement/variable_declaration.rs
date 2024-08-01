use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    lang::{parser::error::ParserError, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}}, node::{expression::{ASTExpression, ExpressionNode},Node, NodeParserTickResult}, vm::tick::VMTick
};

use super::{StatementNode, StatementResult};

/// Defined the method variables are stored
#[derive(Debug, Clone)]
pub enum VariableType {
    /// A variable that can be change
    Variable,

    /// A variable that can't be changed
    Constant
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    /// The type of the variable
    #[allow(dead_code)]
    vartype: VariableType,

    /// The name of the variable
    varname: String,

    /// The value of the variable
    value: Option<ASTExpression>
}

impl Node for VariableDeclaration {
    /// Transcribes the declaration of ONE variable
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> where Self: Sized {
        // Variables should start with the keyword `let`
        cursor.expect(&Token::LangKeyword(TokenLangKeyword::Let), ParserError::Expected("start@var_declaration <keyword:let> 'let'".to_string()))?;

        // The statement is followed by a variable name
        if let Some(Token::CustomKeyword(varname)) = cursor.consume() {
            let varname = varname.clone();

            // Optionally, the variable can be assigned a value (after an equal sign)
            let value = if let Some(&Token::Symbol(TokenSymbol::Equal)) = cursor.peek() {
                cursor.next();
                ASTExpression::transcribe(cursor)?
            } else {
                None
            };

            // By default, variables are mutable (variable)
            Ok(Some(VariableDeclaration {
                vartype: VariableType::Variable,
                varname,
                value
            }))
        } else {
            Err(TranscriptionException::Error(ParserError::Expected("varname@var_declaration <keyword:custom>".to_string())))
        }
    }
}

impl StatementNode for VariableDeclaration {
    /// Creates a new variable for the current context and assigns a value to it.
    /// Returns the value of the variable.
    fn execute(&self, tick: &mut VMTick) -> Option<StatementResult> {
        // Evaluate the expression containing it's value
        let value = self.value.clone().map(|expr| expr.evaluate(tick)).unwrap_or_default();

        // Create a new variable in the context with it's data
        tick.get_context().create(self.varname.clone(), value.clone());

        // [SemiExpression] Return (if any) the value returned by the first node of the scope that returned a value
        Some(StatementResult::Usable(value))
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {}", self.varname)?;

        if let Some(value) = &self.value {
            write!(f, " = {}", value)
        } else {
            Ok(())
        }
    }
}