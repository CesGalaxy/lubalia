use super::token::Token;

/// Examines the tokens searching for errors, bugs or other problems.
/// 
/// Returns `None` if there are no errors. Otherwise, `Some(LinterError)`.
pub fn linter(tokens: &Vec<Token>) -> Option<LinterError> {
    let mut pos = 0;

    while let Some(t) = tokens.get(pos) {
        if *t == Token::Semicolon {
            if tokens.get(pos + 1).cloned() != Some(Token::EOL) {
                return Some(LinterError::SemicolonNotAtEnd);
            }
        }

        pos += 1;
    }

    None
}

#[derive(Debug)]
pub enum LinterError {
    SemicolonNotAtEnd
}