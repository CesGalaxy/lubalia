use crate::lang::token::Token;

/// Examines the tokens searching for errors, bugs or other problems.
/// The linter doesn't distuish from fatal errors and warnings.
/// 
/// Returns `None` if there are no errors. Otherwise, `Some(LinterError)`.
/// 
/// # Panics
/// 
/// Panics if there is an unexcepted error (not related with the code).
pub fn linter(tokens: &Vec<Token>) -> Option<LinterError> {
    let mut pos = 0;

    while let Some(t) = tokens.get(pos) {
        if t == &Token::Semicolon {
            if tokens.get(pos + 1) != Some(&Token::EOL) {
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

impl std::fmt::Display for LinterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SemicolonNotAtEnd => write!(f, "Semicolon not at end"),
        }
    }
}
