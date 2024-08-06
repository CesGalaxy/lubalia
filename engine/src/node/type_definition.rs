use lubalia_utils::{cursor::CursorNavigation, transcriber::{cursor::TranscriberCursor, error::TranscriptionException}};

use crate::{data::types::DataType, lang::{parser::{context::ParsingContext, cursor::ignore_eols, error::{expected_token, ParserError}}, token::{keyword::TokenLangKeyword, symbol::TokenSymbol, Token}}};

use super::{Node, NodeParserTickResult};

impl DataType {
    pub fn transcribe_terminal(cursor: &mut TranscriberCursor<Token>, _ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        let base_type = match cursor.consume() {
            Some(Token::LangKeyword(TokenLangKeyword::Null)) => Ok(Some(DataType::Null)),
            Some(Token::LangKeyword(TokenLangKeyword::True)) => Ok(Some(DataType::True)),
            Some(Token::LangKeyword(TokenLangKeyword::False)) => Ok(Some(DataType::False)),
            Some(Token::CustomKeyword(keyword)) => match keyword.as_str() {
                "num" => Ok(Some(DataType::Number)),
                "str" => Ok(Some(DataType::String)),
                "char" => Ok(Some(DataType::Char)),
                "bool" => Ok(Some(DataType::Boolean)),
                "any" => Ok(Some(DataType::Any)),
                "never" => Ok(Some(DataType::Never)),
                _ => Err(TranscriptionException::Error(ParserError::Expected(expected_token!(<type:keyword>))))
            }
            _ => Err(TranscriptionException::Error(ParserError::Expected(expected_token!(<type>))))
        };

        ignore_eols(cursor);

        // Use ? for optional
        if let Some(Token::Symbol(TokenSymbol::Question)) = cursor.peek() {
            cursor.next();
            base_type.map(|a| a.map(Box::new).map(DataType::Optional))
        } else {
            base_type
        }
    }
}

impl Node for DataType {
    fn transcribe(cursor: &mut TranscriberCursor<Token>, ctx: &mut ParsingContext) -> NodeParserTickResult<Self> {
        let first = DataType::transcribe_terminal(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(<keyword:true>))))?;

        // TODO: Ignore all but last?
        ignore_eols(cursor);

        Ok(Some(if let Some(Token::Symbol(TokenSymbol::Pipe)) = cursor.peek() {
            let mut types = vec![first];

            while let Some(Token::Symbol(TokenSymbol::Pipe)) = cursor.peek() {
                cursor.next();
                ignore_eols(cursor);

                let next = DataType::transcribe_terminal(cursor, ctx)?.ok_or(TranscriptionException::Error(ParserError::Expected(expected_token!(<keyword:true>))))?;

                ignore_eols(cursor);
                types.push(next);
            }

            DataType::Mixed(types)
        } else {
            first
        }))
    }
}
