use std::fmt;

use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException, intent::TranscriptionIntent}};

use crate::{data::DataValue, lang::{parser::{cursor::ignore_eols, error::ParserError}, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}}, node::{expression::{ASTExpression, ExpressionNode}, Node, NodeParserTickResult}, vm::tick::VMTick};

use super::{scope::ScopeStruct, StatementNode, StatementResult};

#[derive(Debug, Clone)]
pub struct SwitchStatement {
    /// The expression to evaluate
    expression: ASTExpression,

    /// The cases to evaluate
    cases: Vec<SwitchCase>
}

impl Node for SwitchStatement {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> where Self: Sized {
        // Switch statements should start with the keyword `if`
        cursor.expect(&Token::LangKeyword(TokenLangKeyword::Switch), ParserError::Expected("start@switch <keyword:switch> 'switch'".to_string()))?;

        // Get the expression to evaluate
        let expression = ASTExpression::transcribe(cursor)?.ok_or(TranscriptionException::Error(ParserError::Expected("expression@switch <expr>".to_string())))?;

        // Then list the cases between braces
        cursor.expect(&Token::Symbol(TokenSymbol::BraceOpen), ParserError::Expected("opening@switch/sym <sym:brace:open> '{'".to_string()))?;

        ignore_eols(cursor);

        let mut cases = vec![];

        // Save all cases found inside the switch until a closing brace is found (and ends the switch)
        while let TranscriptionIntent(Ok(Some(case_node))) = cursor.intent(SwitchCase::transcribe) {
            cases.push(case_node);

            ignore_eols(cursor);
        }

        // Get the closing brace
        cursor.expect(&Token::Symbol(TokenSymbol::BraceClose), ParserError::Expected("closing@switch/sym <sym:brace:close> '}'".to_string()))?;

        Ok(Some(Self { expression, cases }))
    }
}

impl StatementNode for SwitchStatement {
    fn execute(&self, tick: &mut VMTick) -> Option<StatementResult> {
        let main_value = self.expression.evaluate(tick);

        for case in &self.cases {
            if let Some(result) = case.case(tick, &main_value) {
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
pub struct SwitchCase {
    /// The expression to compare
    expression: ASTExpression,

    /// The body of the case
    body: ScopeStruct
}

impl SwitchCase {
    pub fn case(&self, tick: &mut VMTick, main_value: &DataValue) -> Option<Option<StatementResult>> {
        let case_value = self.expression.evaluate(tick);

        // Return Some if matches, None if doesn't
        if main_value == &case_value {
            Some(self.body.execute(tick))
        } else {
            None
        }
    }
}

// Not a node, neither statement, WTF is?
impl Node for SwitchCase {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> NodeParserTickResult<Self> where Self: Sized {
        // Switch cases should start with the keyword `case`
        if cursor.consume() != Some(&Token::LangKeyword(TokenLangKeyword::Case)) {
            return Err(TranscriptionException::Error(ParserError::Expected("start@case <keyword:case> 'case'".to_string())));
        }

        // Get the expression to compare
        let expression = ASTExpression::transcribe(cursor)?.ok_or(TranscriptionException::Error(ParserError::Expected("expression@case <expr>".to_string())))?;

        // Get the body of the case
        let body = ScopeStruct::transcribe(cursor)?.ok_or(TranscriptionException::Error(ParserError::Expected("body@case <scope>".to_string())))?;

        Ok(Some(Self { expression, body }))
    }
}

impl fmt::Display for SwitchCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)?;
        write!(f, "{}", self.body)
    }
}