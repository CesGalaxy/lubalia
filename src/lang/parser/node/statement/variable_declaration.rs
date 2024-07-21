use crate::{
    lang::{parser::{data::DataValue, error::ParserError, node::{expression::{ASTExpression, ExpressionNode}, Node}}, token::{Token, TokenSymbol}},
    utils::transcriber::cursor::TranscriberCursor, vm::VMTick
};

use super::StatementNode;

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
    value: ASTExpression
}

impl Node for VariableDeclaration {
    /// Transcribes the declaration of ONE variable
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized {
        if cursor.consume() != Some(&Token::Keyword("let".to_string())) {
            return Err(ParserError::Expected("start@var_declaration <keyword:let> 'let'".to_string()));
        }

        if let Some(Token::Keyword(varname)) = cursor.consume() {
            let varname = varname.clone();

            if cursor.consume() != Some(&Token::Symbol(TokenSymbol::Equal)) {
                return Err(ParserError::Expected("equal@var_declaration <sym:equal> '='".to_string()));
            }

            let value = ASTExpression::transcribe(cursor)?.ok_or(ParserError::Expected("value@var_declaration <expr>".to_string()))?;

            Ok(Some(VariableDeclaration {
                vartype: VariableType::Variable,
                varname,
                value
            }))
        } else {
            Err(ParserError::Expected("varname@var_declaration".to_string()))
        }
    }
}

impl StatementNode for VariableDeclaration {
    /// Creates a new variable for the current context and assigns a value to it.
    /// Returns the value of the variable.
    fn execute(&self, tick: &mut VMTick) -> Option<DataValue> {
        let value = self.value.evaluate(tick);

        tick.get_context().create(self.varname.clone(), value.clone());

        Some(value)
    }
}