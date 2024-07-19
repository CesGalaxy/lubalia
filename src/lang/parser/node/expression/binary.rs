use crate::lang::parser::data::DataValue;
use crate::lang::parser::{error::ParserError, node::Node};
use crate::lang::token::{Token, TokenSymbol};
use crate::utils::transcriber::cursor::TranscriberCursor;
use crate::vm::context::Context;
use super::terminal::TerminalExpression;
use super::{ASTExpression, ExpressionNode};

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

impl From<&Operator> for u8 {
    fn from(value: &Operator) -> Self {
        match value {
            Operator::Add | Operator::Sub => 0,
            Operator::Mul | Operator::Div => 1
        }
    }
}

// impl PartialOrd for Operator {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match (self, other) {
//             (Self::Mul, Self::Div) | (Self::Div, Self::Mul) => Some(std::cmp::Ordering::Equal),
//             (Self::Add, Self::Sub) | (Self::Sub, Self::Add) => Some(std::cmp::Ordering::Equal),
//             (Self::Mul, _) | (Self::Div, _) => Some(std::cmp::Ordering::Greater),
//             (Self::Add, _) | (Self::Sub, _) => Some(std::cmp::Ordering::Less),
//             _ => Some(std::cmp::Ordering::Equal)
//         }
//     }
// }

impl From<&Token> for Option<Operator> {
    fn from(value: &Token) -> Self {
        match value {
            Token::Symbol(TokenSymbol::Plus) => Some(Operator::Add),
            Token::Symbol(TokenSymbol::Minus) => Some(Operator::Sub),
            Token::Symbol(TokenSymbol::Asterisk) => Some(Operator::Mul),
            Token::Symbol(TokenSymbol::Slash) => Some(Operator::Div),
            _ => None
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BinaryExpression {
    lhs: Box<ASTExpression>,
    operator: Operator,
    rhs: Box<ASTExpression>
}

// pub fn transcribe_next_segment(
//     cursor: &mut TranscriberCursor<Token>,
//     stack: Vec<(Box<ASTExpression>, Operator)>,
// ) -> Result<Option<BinaryExpression>, ParserError> {
//     expr:terminal + sym/operator + ...
//     let v = Box::new(ASTExpression::Terminal(
//         TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:segment|expr:terminal>".to_string()))?
//     ));
// 
//     let o: Option<Operator> = cursor.consume().and_then(|token| token.into());
// 
//     // let mut stack = stack.iter().rev();
//    
//     if let Some(o) = o {
//         let nv = Box::new(ASTExpression::Terminal(
//             TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:segment|expr:terminal>".to_string()))?
//         ));
// 
//         let mut buff = None;
// 
//         if let Some((pv, po)) = stack.next() {
//             if u8::from(po) < u8::from(&o) {
//                 buff = Some(BinaryExpression {
//                     lhs: pv.clone(),
//                     operator: o.clone(),
//                     rhs: Box::new(ASTExpression::Binary(BinaryExpression {
//                         lhs: v.clone(),
//                         operator: o.clone(),
//                         rhs: nv.clone(),
//                     })),
//                 });
//             } else {
//                 // (( pv po v ) o nv)
//                 buff = Some(BinaryExpression {
//                     lhs: Box::new(ASTExpression::Binary(BinaryExpression {
//                         lhs: pv.clone(),
//                         operator: po.clone(),
//                         rhs: v.clone(),
//                     })),
//                     operator: o.clone(),
//                     rhs: v
//                 });
//             }
//         }
// 
//         let new_stack = vec![];
// 
//         while let Some(segment) = stack.next
// 
//         Ok(buff)
//     } else {
//         let mut buff: Option<BinaryExpression> = None;
// 
//         while let Some((pv, po)) = stack.next() {
//             buff = Some(BinaryExpression {
//                 lhs: pv.clone(),
//                 operator: po.clone(),
//                 rhs: if let Some(b) = buff { Box::new(ASTExpression::Binary(b)) } else { v.clone() },
//             });
//         }
// 
//         Ok(buff)
//     }
// }

impl Node for BinaryExpression {
    fn transcribe(cursor: &mut TranscriberCursor<Token>) -> Result<Option<BinaryExpression>, ParserError> {
        let v1 = Box::new(ASTExpression::Terminal(
            TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal>".to_string()))?
        ));

        let o1: Operator = cursor.consume().and_then(|token| token.into()).ok_or(ParserError::Expected("<sym/operator>".to_string()))?;

        let v2 = Box::new(ASTExpression::Terminal(
            TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + sym/operator + ...|expr:terminal>".to_string()))?
        ));

        let o2: Option<Operator> = cursor.peek().and_then(|token| token.into());

        match o2 {
            Some(o2) => {
                cursor.next();

                if u8::from(&o1) < u8::from(&o2) {
                    let v3 = Box::new(ASTExpression::Terminal(
                        TerminalExpression::transcribe(cursor)?.ok_or(ParserError::Expected("<expr:terminal + sym/operator + ...|expr:terminal>".to_string()))?
                    ));

                    let o3: Option<Operator> = cursor.peek().and_then(|token| token.into());

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
    fn evaluate(&self, context: &mut Context) -> DataValue {
        let lhs = self.lhs.evaluate(context);
        let rhs = self.rhs.evaluate(context);

        match self.operator {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs
        }
    }
}