use crate::{
    lang::{self, parser::{self, data::DataValue, error::ParserError, node::{expression::{ASTExpression, ExpressionNode}, Node}}, token::Token},
    utils::transcriber::cursor::TranscriberCursor, vm::{context::Context, VM}
};

use super::StatementNode;

#[derive(Debug, Clone)]
pub enum VariableType {
    Variable,
    Constant
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    #[allow(dead_code)]
    vartype: VariableType,
    varname: String,
    value: Option<ASTExpression>
}

impl Node for VariableDeclaration {
    fn transcribe(cursor: &mut TranscriberCursor<lang::token::Token>) -> Result<Option<Self>, parser::error::ParserError> where Self: Sized {
        if !cursor.consume().is_some_and(|t| t == &Token::Keyword("let".to_string())) {
            return Err(ParserError::Expected("start@var_declaration <keyword:let> 'let'".to_string()));
        }

        if let Some(Token::Keyword(varname)) = cursor.consume() {
            let varname = varname.clone();

            if !cursor.consume().is_some_and(|t| t == &Token::Symbol(lang::token::TokenSymbol::Equal)) {
                return Err(ParserError::Expected("equal@var_declaration <sym:equal> '='".to_string()));
            }

            let value = ASTExpression::transcribe(cursor)?;

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
    fn execute(&self, context: &mut Context, vm: &mut VM) -> Result<(), &'static str> {
        let value = self.value.clone().map(|v| v.evaluate(context, vm)).unwrap_or(DataValue::Null);

        context.set(self.varname.clone(), value);

        Ok(())
    }
}