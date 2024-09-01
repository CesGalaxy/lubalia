use std::{cell::RefCell, fmt};

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{
    data::DataValue,
    lang::{
        parser::{context::ParsingContext, cursor::ignore_eols, error::{expected_token, ParserError}},
        syntax::node::{expression::ExpressionNode, ASTNode, Node, NodeParserTickResult},
        token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}
    },
    vm::{scope::Scope, VM}
};

use super::{StatementNode, StatementResult};

#[derive(Debug, Clone)]
pub struct SwitchStatement {
    /// The expression to evaluate
    expression: Box<ASTNode>,

    /// The cases to evaluate
    cases: Vec<SwitchCase>
}

impl Node for SwitchStatement {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Switch statements should start with the keyword `if`
        cursor.expect(&Token::Keyword(TokenLangKeyword::Switch), ParserError::Expected("start@switch <keyword:switch> 'switch'".to_string()))?;

        ignore_eols(cursor);

        // Get the expression to evaluate
        let expression = Box::new(ASTNode::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("expression@switch <node>".to_string())))?);

        ignore_eols(cursor);

        // Then list the cases between braces
        cursor.expect(&Token::Symbol(TokenSymbol::BraceOpen), ParserError::Expected("opening@switch/sym <sym:brace:open> '{'".to_string()))?;

        ignore_eols(cursor);

        let mut cases = vec![];

        // Save all cases found inside the switch until a closing brace is found (and ends the switch)
        while let Some(Token::Keyword(TokenLangKeyword::Case)) = cursor.peek() {
            cases.push(SwitchCase::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected("case@switch <case>".to_string())))?);

            ignore_eols(cursor);
        }

        // Get the closing brace
        cursor.expect(&Token::Symbol(TokenSymbol::BraceClose), ParserError::Expected("closing@switch/sym <sym:brace:close> '}'".to_string()))?;

        Ok(Some(Self { expression, cases }))
    }
}

impl StatementNode for SwitchStatement {
    fn execute(&self, vm: &mut VM, scope: &RefCell<Scope>) -> Option<StatementResult> {
        let main_value = self.expression.evaluate(vm, scope);

        for case in &self.cases {
            if let Some(result) = case.case(vm, scope, &main_value) {
                return result;
            }
        }

        None
    }
}

impl fmt::Display for SwitchStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "switch {} {{\n", self.expression)?;

        for case in &self.cases {
            write!(f, "> {}\n", case)?;
        }

        write!(f, "}}")
    }
}

#[derive(Debug, Clone)]
struct SwitchCase {
    /// The expression to compare
    case: Option<ASTNode>,

    /// The body of the case
    body: ASTNode
}

impl SwitchCase {
    fn case(&self, vm: &mut VM, scope: &RefCell<Scope>, main_value: &DataValue) -> Option<Option<StatementResult>> {
        let case = self.case.as_ref().map(|node| node.evaluate(vm, scope));

        // Return Some if matches, None if doesn't
        if case.map(|case_value| main_value == &case_value).unwrap_or(true) {
            Some(self.body.execute(vm, scope))
        } else {
            None
        }
    }
}

// Not a node, neither statement, WTF is?
impl Node for SwitchCase {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> where Self: Sized {
        // Switch cases should start with the keyword `case`
        if cursor.consume() != Some(&Token::Keyword(TokenLangKeyword::Case)) {
            return Err(TranscriptionException::Error(ParserError::Expected(expected_token!(start@case <keyword:case>))));
        }

        ignore_eols(cursor);

        // Get the expression to compare
        let expression = if let Some(Token::Keyword(TokenLangKeyword::Default)) = cursor.peek() {
            cursor.next();
            None
        } else {
            Some(ASTNode::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(expression@case <node>))))?)
        };

        ignore_eols(cursor);

        // Get the body of the case
        let body = ASTNode::transcribe(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(body@case <node>))))?;

        Ok(Some(Self { case: expression, body }))
    }
}

impl fmt::Display for SwitchCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.case)?;
        write!(f, "{}", self.body)
    }
}