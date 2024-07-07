use crate::lang::lexer::token::Token;

/// A tool for moving through a vector of tokens.
pub struct ParsingMachine {
    /// The vector of tokens
    pub tokens: Vec<Token>,
    /// The position of the cursor in the vector of tokens
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

    /// Get the current token (if posible)
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Get the next token (if posible)
    pub fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    /// Get the previous token (if posible)
    pub fn peek_back(&self) -> Option<&Token> {
        self.tokens.get(self.pos - 1)
    }

    // Get the current token (if posible), and then advance the cursor to the next one
    pub fn consume(&mut self) -> Option<Token> {
        let token = match self.tokens.get(self.pos) {
            Some(token) => Some(token.clone()),
            None => return None,
        };

        self.pos += 1;
        token
    }

    /// Move the cursor to the next token, returns if the cursor has overflown
    pub fn next(&mut self) -> bool {
        self.pos += 1;
        self.is_overflow()
    }

    /// Move the cursor to the previous token, returns if the cursor has underflown
    pub fn back(&mut self) -> bool {
        self.pos -= 1;
        self.is_overflow()
    }

    /// Returns if the cursor has overflown (the position exceeded the vector size/length)
    pub fn is_overflow(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}