use crate::{
    lang::{parser::{error::ParserError, node::{expression::{terminal::TerminalExpression, ASTExpression}, Node}}, token::Token},
    utils::transcriber::cursor::TranscriberCursor
};

use super::{operator::BinaryOperator, BinaryExpression};

impl Node for BinaryExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<BinaryExpression>, ParserError> {
        let v1 = Box::new(ASTExpression::Terminal(
            TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal>".to_string()))?
        ));

        let o1: BinaryOperator = cursor.consume().and_then(|token| token.into()).ok_or(ParserError::Expected("<operator>".to_string()))?;

        let v2 = Box::new(ASTExpression::Terminal(
            TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + operator + ...|expr:terminal>".to_string()))?
        ));

        let o2: Option<BinaryOperator> = cursor.peek().and_then(|token| token.into());

        match o2 {
            Some(o2) => {
                cursor.next();

                if u8::from(&o1) < u8::from(&o2) {
                    let v3 = Box::new(ASTExpression::Terminal(
                        TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + operator + ...|expr:terminal>".to_string()))?
                    ));

                    let o3: Option<BinaryOperator> = cursor.peek().and_then(|token| token.into());

                    match o3 {
                        Some(o3) => {
                            cursor.next();
                            
                            let v4 = Box::new(ASTExpression::Terminal(
                                TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + operator + ...|expr:terminal>".to_string()))?
                            ));

                            if u8::from(&o1) < u8::from(&o3) {
                                let lhs = v1;

                                let operator = o1;

                                let lhs2 = Box::new(ASTExpression::Binary(Self {
                                    lhs: v2,
                                    operator: o2,
                                    rhs: v3
                                }));

                                let operator2 = o3;

                                let rhs2 = v4;

                                let rhs = Box::new(ASTExpression::Binary(Self {
                                    lhs: lhs2,
                                    operator: operator2,
                                    rhs: rhs2
                                }));

                                Ok(Some(Self { lhs, operator, rhs }))
                            } else {
                                // ( ( v1 + ( v2 * v3 ) ) + v4 )
                                let lhs2 = v1;

                                let operator2 = o1;

                                let rhs2 = Box::new(ASTExpression::Binary(Self {
                                    lhs: v2,
                                    operator: o2,
                                    rhs: v3
                                }));

                                let lhs = Box::new(ASTExpression::Binary(Self {
                                    lhs: lhs2,
                                    operator: operator2,
                                    rhs: rhs2
                                }));

                                let operator = o3;

                                let rhs = v4;

                                Ok(Some(Self { lhs, operator, rhs }))
                            }
                        },
                        _ => {
                            let lhs = v1;

                            let operator = o1;

                            let rhs = Box::new(ASTExpression::Binary(Self {
                                lhs: v2,
                                operator: o2,
                                rhs: v3
                            }));

                            Ok(Some(Self { lhs, operator, rhs }))
                        }
                    }
                } else {
                    let lhs = Box::new(ASTExpression::Binary(Self {
                        lhs: v1,
                        operator: o1,
                        rhs: v2
                    }));

                    let operator = o2;

                    let rhs = Box::new(
                        ASTExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr>".to_string()))?
                    );

                    Ok(Some(Self { lhs, operator, rhs }))
                }
            },
            None => Ok(Some(Self {
                lhs: v1,
                operator: o1,
                rhs: v2
            }))
        }
    }
}