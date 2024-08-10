use std::{cell::RefCell, fmt};

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    data::types::DataType,
    lang::{
        parser::{context::ParsingContext, cursor::ignore_eols, error::ParserError},
        syntax::node::{expression::ExpressionNode, ASTNode, Node, NodeParserTickResult},
        token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}
    },
    vm::{scope::Scope, VM}
};

use super::{StatementNode, StatementResult};

/// Defined the method variables are stored
#[derive(Debug, Clone)]
pub enum VariableMutability {
    /// A variable that can be change
    Variable,

    /// A variable that can't be changed
    Constant
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    /// The mutability of the variable
    #[allow(dead_code)]
    varmut: VariableMutability,

    /// The type of the variable
    vartype: DataType,

    /// The name of the variable
    varname: String,

    /// The value of the variable
    value: Option<Box<ASTNode>>
}

impl Node for VariableDeclaration {
    /// Transcribes the declaration of ONE variable
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Variables should start with the keyword `let`
        cursor.expect(&Token::LangKeyword(TokenLangKeyword::Let), ParserError::Expected("start@var_declaration <keyword:let> 'let'".to_string()))?;

        ignore_eols(cursor);

        // The statement is followed by a variable name
        if let Some(Token::CustomKeyword(varname)) = cursor.consume() {
            let varname = varname.clone();

            ignore_eols(cursor);

            let vartype = if let Some(Token::Symbol(TokenSymbol::Colon)) = cursor.peek() {
                cursor.next();
                ignore_eols(cursor);

                let vartype = DataType::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("vartype@var_declaration <type>".to_string())))?;

                ignore_eols(cursor);

                vartype
            } else {
                DataType::Any
            };

            // Optionally, the variable can be assigned a value (after an equal sign)
            let value = if let Some(&Token::Symbol(TokenSymbol::Equal)) = cursor.peek() {
                cursor.next();
                ignore_eols(cursor);
                // TODO: What I was even thinking when I wrote this?
                ASTNode::transcribe(cursor, ctx)?
            } else {
                // Keep last EOL
                cursor.back();
                None
            }.map(Box::new);

            // By default, variables are mutable (variable)
            Ok(Some(VariableDeclaration { varmut: VariableMutability::Variable, vartype, varname, value }))
        } else {
            Err(TranscriptionException::Error(ParserError::Expected("varname@var_declaration <keyword:custom>".to_string())))
        }
    }
}

impl StatementNode for VariableDeclaration {
    /// Creates a new variable for the current context and assigns a value to it.
    /// Returns the value of the variable.
    fn execute(&self, vm: &mut VM, scope: &RefCell<Scope>) -> Option<StatementResult> {
        // Evaluate the expression containing it's value
        let value = self.value.clone().map(|node| node.evaluate(vm, scope)).unwrap_or_default();

        // Create a new variable in the context with it's data
        scope.borrow_mut().create(self.varname.clone(), (value.clone(), self.vartype.clone()));

        // [SemiExpression] Return (if any) the value returned by the first node of the scope that returned a value
        Some(StatementResult::Usable(value))
    }
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if let VariableMutability::Variable = self.varmut { "var" } else { "const" })?;

        write!(f, " {}", self.varname)?;

        if &self.vartype != &DataType::Any {
            write!(f, ": {}", self.vartype)?;
        }

        if let Some(value) = &self.value {
            write!(f, " = {value}")
        } else {
            Ok(())
        }
    }
}