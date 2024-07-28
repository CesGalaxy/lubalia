use lubalia_utils::transcriber::{cursor::TranscriberCursor, transcriber, TranscriberTickResult};

const EXAMPLE_CODE: &str = r#"1234567890"#;

#[derive(Debug)]
enum Token {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

fn main() {
    let transcription = transcriber(EXAMPLE_CODE.chars().collect(), tick);

    match transcription {
        Ok(transcription) => println!("Transcription completed:\n{transcription:?}"),
        Err(error) => println!("Transcription error:\n{error}")
    }
}

fn tick(cursor: &mut TranscriberCursor<char>, initial_char: &char) -> TranscriberTickResult<Token, &'static str> {
    // The cursor moves automatically, but you should do it manually for setting the length of the source of the token
    cursor.next();

    match initial_char {
        '1' => Ok(Some(Token::One)),
        '2' => Ok(Some(Token::Two)),
        '3' => Ok(Some(Token::Three)),
        '4' => Ok(Some(Token::Four)),
        '5' => Ok(Some(Token::Five)),
        '6' => Ok(Some(Token::Six)),
        '7' => Ok(Some(Token::Seven)),
        '8' => Ok(Some(Token::Eight)),
        '9' => Ok(Some(Token::Nine)),
        '0' => Ok(Some(Token::Zero)),
        _ => Err("Invalid token"),
    }
}