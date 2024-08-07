pub mod variable_declaration;
pub mod conditional;
pub mod repeat;
pub mod switch;
pub mod func_call;

use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    data::DataValue,
    lang::{parser::{context::ParsingContext, error::expected_token}, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}},
    vm::tick::VMTick
};

use super::{block::BlockStruct, expression::ExpressionNode, ASTNode, Node, NodeParserTickResult};

/// Wether the statement returned a value for using it or the result is just a side effect
#[derive(Debug, Clone)]
pub enum StatementResult {
    /// Something was returned by the program
    Return(DataValue),

    /// The statement result, it's not returned
    Usable(DataValue)
}

impl StatementResult {
    /// Get the value of the statement result
    pub fn value(&self) -> DataValue {
        match self {
            StatementResult::Return(value) => value.clone(),
            StatementResult::Usable(value) => value.clone()
        }
    }

    /// Get the returned value (if any)
    pub fn returned(&self) -> Option<DataValue> {
        match self {
            StatementResult::Return(value) => Some(value.clone()),
            StatementResult::Usable(_) => None
        }
    }
}

/// An instruction the VM executes without returning a value
#[derive(Debug, Clone)]
pub enum ASTStatement {
    VariableDeclaration(variable_declaration::VariableDeclaration),
    Block(BlockStruct),
    Conditional(conditional::ConditionalStatement),
    Repeat(repeat::Repeat),
    Switch(switch::SwitchStatement),
    FunctionCall(func_call::FunctionCallStatement),
    Return(Box<ASTNode>)
}

pub trait StatementNode: Node {
    fn execute(&self, tick: &mut VMTick) -> Option<StatementResult>;
}

impl Node for ASTStatement {
    /// Transcribe an statement (if possible)
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        //? Should this return Err if no statement is found? So node transcription ignores all errors and tries an expr (which will the one that can fail)
        //* This must make sure that the transcribed node is the correct one. In case of error, it will fail.
        match cursor.peek() {
            // Statements are usually defined with an initial keyword
            Some(Token::LangKeyword(keyword)) => match keyword {
                TokenLangKeyword::Let => variable_declaration::VariableDeclaration::transcribe(cursor, ctx).map(|vd| vd.map(ASTStatement::VariableDeclaration)),
                TokenLangKeyword::If => conditional::ConditionalStatement::transcribe(cursor, ctx).map(|cond| cond.map(ASTStatement::Conditional)),
                TokenLangKeyword::Repeat => repeat::Repeat::transcribe(cursor, ctx).map(|repeat| repeat.map(ASTStatement::Repeat)),
                TokenLangKeyword::Switch => switch::SwitchStatement::transcribe(cursor, ctx).map(|switch| switch.map(ASTStatement::Switch)),
                TokenLangKeyword::Return => {
                    cursor.next();
                    ASTNode::transcribe(cursor, ctx).map(|expr| expr.map(Box::new).map(ASTStatement::Return))
                },
                _ => Err(TranscriptionException::NotFound(expected_token!(LangKeyword; <stmnt>)))
            },
            Some(Token::CustomKeyword(_)) => if let Some(Token::Symbol(TokenSymbol::ParenOpen)) = cursor.peek_next() {
                func_call::FunctionCallStatement::transcribe(cursor, ctx).map(|call| call.map(ASTStatement::FunctionCall))
            } else {
                Err(TranscriptionException::NotFound(expected_token!(<stmnt>)))
            },
            // Scopes are statements too
            Some(Token::Symbol(TokenSymbol::BraceOpen)) => BlockStruct::transcribe(cursor, ctx).map(|scope| scope.map(Self::Block)),
            _ => Err(TranscriptionException::NotFound(expected_token!(<stmnt>)))
        }
    }
}

impl StatementNode for ASTStatement {
    /// Execute an statement and return a value if any is provided
    fn execute(&self, tick: &mut VMTick) -> Option<StatementResult> {
        match self {
            ASTStatement::VariableDeclaration(vd) => vd.execute(tick),
            ASTStatement::Block(block) => block.execute(tick),
            ASTStatement::Conditional(cond) => cond.execute(tick),
            ASTStatement::Repeat(repeat) => repeat.execute(tick),
            ASTStatement::Switch(switch) => switch.execute(tick),
            ASTStatement::FunctionCall(call) => call.execute(tick),
            ASTStatement::Return(node) => Some(StatementResult::Return(node.evaluate(tick)))
        }
    }
}

impl ExpressionNode for ASTStatement {
    /// Evaluate the statement and return the result value
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        self.execute(tick).map(|result| result.value()).unwrap_or_default()
    }
}

impl fmt::Display for ASTStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTStatement::VariableDeclaration(vd) => write!(f, "{vd}"),
            ASTStatement::Block(scope) => write!(f, "{scope}"),
            ASTStatement::Conditional(cond) => write!(f, "{cond}"),
            ASTStatement::Repeat(repeat) => write!(f, "{repeat}"),
            ASTStatement::Switch(switch) => write!(f, "{switch}"),
            ASTStatement::FunctionCall(call) => write!(f, "{call}"),
            ASTStatement::Return(node) => write!(f, "return ( {node} )")
        }
    }
}