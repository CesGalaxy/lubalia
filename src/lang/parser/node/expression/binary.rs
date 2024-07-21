use std::ops::Not;

use crate::{lang::{parser::{data::{arithmetic::ArithmeticValue, DataValue}, error::ParserError, node::Node}, token::{Token, TokenSymbol}}, utils::transcriber::cursor::TranscriberCursor, vm::VMTick};

use super::{terminal::TerminalExpression, ASTExpression, ExpressionNode};

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    // TODO: I'm too lazy for implementing operators with 2 symbols
    // NoEqual,
    Greater,
    // GreaterOrEqual,
    Less,
    // LessOrEqual,
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    XNOR
}

impl From<&BinaryOperator> for u8 {
    fn from(value: &BinaryOperator) -> Self {
        match value {
            BinaryOperator::AND | BinaryOperator::OR | BinaryOperator::NAND | BinaryOperator::NOR | BinaryOperator::XOR | BinaryOperator::XNOR => 3,
            BinaryOperator::Equal | BinaryOperator::Greater | BinaryOperator::Less => 2,
            BinaryOperator::Mul | BinaryOperator::Div => 1,
            BinaryOperator::Add | BinaryOperator::Sub => 0,
        }
    }
}

impl From<&Token> for Option<BinaryOperator> {
    fn from(value: &Token) -> Self {
        match value {
            Token::Symbol(TokenSymbol::Plus) => Some(BinaryOperator::Add),
            Token::Symbol(TokenSymbol::Minus) => Some(BinaryOperator::Sub),
            Token::Symbol(TokenSymbol::Asterisk) => Some(BinaryOperator::Mul),
            Token::Symbol(TokenSymbol::Slash) => Some(BinaryOperator::Div),
            Token::Symbol(TokenSymbol::Equal) => Some(BinaryOperator::Equal),
            Token::Symbol(TokenSymbol::GreaterThan) => Some(BinaryOperator::Greater),
            Token::Symbol(TokenSymbol::LessThan) => Some(BinaryOperator::Less),
            Token::Symbol(TokenSymbol::Ampersand) => Some(BinaryOperator::AND),
            Token::Symbol(TokenSymbol::Pipe) => Some(BinaryOperator::OR),
            Token::Keyword(keyword) => match keyword.to_ascii_lowercase().as_str() {
                "and" => Some(BinaryOperator::AND),
                "or" => Some(BinaryOperator::OR),
                "nand" => Some(BinaryOperator::NAND),
                "nor" => Some(BinaryOperator::NOR),
                "xor" => Some(BinaryOperator::XOR),
                "xnor" => Some(BinaryOperator::XNOR),
                _ => None
            },
            _ => None
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BinaryExpression {
    lhs: Box<ASTExpression>,
    operator: BinaryOperator,
    rhs: Box<ASTExpression>
}

impl Node for BinaryExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<BinaryExpression>, ParserError> {
        let v1 = Box::new(ASTExpression::Terminal(
            TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal>".to_string()))?
        ));

        let o1: BinaryOperator = cursor.consume().and_then(|token| token.into()).ok_or(ParserError::Expected("<sym/operator>".to_string()))?;

        let v2 = Box::new(ASTExpression::Terminal(
            TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + sym/operator + ...|expr:terminal>".to_string()))?
        ));

        let o2: Option<BinaryOperator> = cursor.peek().and_then(|token| token.into());

        match o2 {
            Some(o2) => {
                cursor.next();

                if u8::from(&o1) < u8::from(&o2) {
                    let v3 = Box::new(ASTExpression::Terminal(
                        TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + sym/operator + ...|expr:terminal>".to_string()))?
                    ));

                    let o3: Option<BinaryOperator> = cursor.peek().and_then(|token| token.into());

                    match o3 {
                        Some(o3) => {
                            cursor.next();
                            
                            let v4 = Box::new(ASTExpression::Terminal(
                                TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + sym/operator + ...|expr:terminal>".to_string()))?
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

impl ExpressionNode for BinaryExpression {
    fn evaluate(&self, tick: &mut VMTick) -> DataValue {
        let lhs = self.lhs.evaluate(tick);
        let rhs = self.rhs.evaluate(tick);

        match self.operator {
            BinaryOperator::Add => (ArithmeticValue::from(lhs) + ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Sub => (ArithmeticValue::from(lhs) - ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Mul => (ArithmeticValue::from(lhs) * ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Div => (ArithmeticValue::from(lhs) / ArithmeticValue::from(rhs)).into(),
            BinaryOperator::Equal => (lhs == rhs).into(),
            BinaryOperator::Greater => (lhs > rhs).into(),
            BinaryOperator::Less => (lhs < rhs).into(),
            BinaryOperator::AND => (bool::from(lhs) && bool::from(rhs)).into(),
            BinaryOperator::OR => (bool::from(lhs) || bool::from(rhs)).into(),
            BinaryOperator::NAND => (!bool::from(lhs) || !bool::from(rhs)).into(),
            BinaryOperator::NOR => (!bool::from(lhs) && !bool::from(rhs)).into(),
            BinaryOperator::XOR => (bool::from(lhs) ^ bool::from(rhs)).into(),
            BinaryOperator::XNOR => (bool::from(lhs) ^ bool::from(rhs)).not().into()
        }
    }
}

impl std::fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} {:?} {} )", self.lhs, self.operator, self.rhs)
    }
}