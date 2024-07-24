use crate::{
    engine::{data::DataValue, node::{expression::{ASTExpression, ExpressionNode}, Node}},
    lang::{parser::error::ParserError, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}},
    utils::transcriber::cursor::TranscriberCursor,
    vm::VMTick
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
    value: Option<ASTExpression>
}

impl Node for VariableDeclaration {
    /// Transcribes the declaration of ONE variable
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<Self>, ParserError> where Self: Sized {
        if cursor.consume() != Some(&Token::LangKeyword(TokenLangKeyword::Let)) {
            return Err(ParserError::Expected("start@var_declaration <keyword:let> 'let'".to_string()));
        }

        if let Some(Token::CustomKeyword(varname)) = cursor.consume() {
            let varname = varname.clone();

            let value = if let Some(&Token::Symbol(TokenSymbol::Equal)) = cursor.peek() {
                cursor.next();
                ASTExpression::transcribe(cursor)?
            } else {
                None
            };

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
        let value = self.value.clone().map(|expr| expr.evaluate(tick)).unwrap_or_default();

        tick.get_context().create(self.varname.clone(), value.clone());

        Some(value)
    }
}

impl std::fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {}", self.varname)?;
        if let Some(value) = &self.value {
            write!(f, " = {}", value)
        } else {
            Ok(())
        }
    }
}