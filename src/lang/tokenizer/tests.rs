use super::*;

static TEST_CODE: &'static str = "let a = 10;\nlet b = 20;\nlet c = \"Hello, World!\";";

#[test]
fn test_tokenization() {
    let transcription = tokenizer(TEST_CODE.to_string()).unwrap();

    assert_eq!(transcription.units().len(), 19, "the transcription should have 19 units");

    assert_eq!(transcription.units(), vec![
        &Token::Keyword("let".to_string()),
        &Token::Keyword("a".to_string()),
        &Token::Symbol(TokenSymbol::Equal),
        &Token::Literal(TokenLiteral::Number(10.0)),
        &Token::Semicolon,
        &Token::EOL,
        &Token::Keyword("let".to_string()),
        &Token::Keyword("b".to_string()),
        &Token::Symbol(TokenSymbol::Equal),
        &Token::Literal(TokenLiteral::Number(20.0)),
        &Token::Semicolon,
        &Token::EOL,
        &Token::Keyword("let".to_string()),
        &Token::Keyword("c".to_string()),
        &Token::Symbol(TokenSymbol::Equal),
        &Token::Literal(TokenLiteral::String("Hello, World!".to_string())),
        &Token::Semicolon,
        &Token::EOL,
        &Token::EOF,
    ], "the transcription units should be the expected ones");
}

#[test]
fn test_tokenization_details() {
    let transcription = tokenizer(TEST_CODE.to_string()).unwrap();

    assert!(transcription.completed, "the transcription should be completed");
    assert_eq!(transcription.source, TEST_CODE.chars().collect::<Vec<_>>() , "the transcription source should be the same as the input source");
}
