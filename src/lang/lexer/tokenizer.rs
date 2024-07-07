use super::{token::{Token, TokenLiteral, TokenSymbol}, LexerResult};

/// Returns a vector of tokens from the given code.
/// 
/// # Panics
/// 
/// Panics if there is an unexcepted error (not related with the code).
pub fn tokenizer(code: String) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while let Some(c) = code.chars().nth(pos) {
        if c == ' ' || c == '\t' {
            pos += 1;
            continue;
        } else if c.is_ascii_alphabetic() {
            let mut keyword = String::new();
            keyword.push(c);
            pos += 1;

            while let Some(c) = code.chars().nth(pos) {
                if c == ' ' {
                    break;
                } else if !c.is_ascii_alphanumeric() {
                    return Err(TokenizerError::UnexcepedSymbolAtKeyword(keyword, c));
                }

                keyword.push(c);
                pos += 1;
            }

            tokens.push(Token::Keyword(keyword));
        } else if c.is_numeric() {
            let mut literal = String::new();
            literal.push(c);
            pos += 1;

            while let Some(c) = code.chars().nth(pos) {
                if !c.is_numeric() {
                    break;
                }

                literal.push(c);
                pos += 1;
            }

            tokens.push(Token::Literal(TokenLiteral::Number(literal.parse().or_else(|_| Err(TokenizerError::ErrorParsingNumber(literal)))?)));
            continue;
        } else if c == '"' {
            let mut literal = String::new();
            pos += 1;

            while let Some(c) = code.chars().nth(pos) {
                if c == '"' {
                    break;
                }

                literal.push(c);
                pos += 1;
            }

            tokens.push(Token::Literal(TokenLiteral::String(literal)));
        } else {
            if let Some(symbol) = TokenSymbol::from_char(c) {
                tokens.push(Token::Symbol(symbol));
            } else {
                match c {
                    ';' => tokens.push(Token::Semicolon),
                    '\n' => tokens.push(Token::EOL),
                    _ => return Err(TokenizerError::UnknownCharacter(c))
                }
            }
        }

        pos += 1;
    }

    tokens.push(Token::EOL);
    tokens.push(Token::EOF);

    Ok(tokens)
}

#[derive(Debug)]
pub enum TokenizerError {
    UnexcepedSymbolAtKeyword(String, char),
    UnknownCharacter(char),
    ErrorParsingNumber(String),
}

impl From<TokenizerError> for LexerResult {
    fn from(value: TokenizerError) -> Self {
        LexerResult::TokenizerError(value)
    }
}
