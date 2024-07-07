use crate::lang::lexer::token::Token;

pub struct ParsingMachine {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

#[allow(dead_code)]
impl ParsingMachine {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    pub fn peek_back(&self) -> Option<&Token> {
        self.tokens.get(self.pos - 1)
    }

    pub fn consume(&mut self) -> Option<Token> {
        let token = match self.tokens.get(self.pos) {
            Some(token) => Some(token.clone()),
            None => return None,
        };

        self.pos += 1;
        token
    }

    pub fn next(&mut self) -> bool {
        self.pos += 1;
        self.is_overflow()
    }

    pub fn back(&mut self) {
        self.pos -= 1;
    }

    pub fn is_overflow(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

// impl Iterator for ParsingMachine {
//     type Item = Token;
//     fn next(&mut self) -> Option<Self::Item> {
//         let token = match self.tokens.get(self.pos) {
//             Some(token) => Some(token.clone()),
//             None => return None,
//         };
//
//         self.pos += 1;
//         token
//     }
// }
//
// impl DoubleEndedIterator for ParsingMachine {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.pos -= 1;
//         let token = match self.tokens.get(self.pos) {
//             Some(token) => Some(token.clone()),
//             None => return None,
//         };
//         token
//     }
// }
//
// impl ExactSizeIterator for ParsingMachine {}