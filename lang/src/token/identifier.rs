macro_rules! identify_token {
    (
        $($segment:ident@$block:ident)|* <$($token_type:ident):+> $($extra:expr)?
    ) => {{
        use colored::Colorize;
        use crate::token::keyword::TokenLangKeyword;

        let components: Vec<String> = vec![$(
            format!("{}@{}", stringify!($segment).yellow(), stringify!($block).yellow())
        ),*];

        let components = components.join("/".black().to_string().as_str());

        let token_type_display = vec!( $( stringify!($token_type).bright_green().to_string() ),+ ).join(":".black().to_string().as_str());

        let token_type_vec = vec!( $( stringify!($token_type) ),+ );
        let mut token_type = token_type_vec.iter();

        let literal = match token_type.next() {
            Some(&"keyword") => if let Some(keyword) = token_type.next() {
                TokenLangKeyword::from_string(keyword).map(|k| <&'static str as From<&TokenLangKeyword>>::from(&k))
            } else {
                None
            },
            Some(&"abc") => Some("ab"),
            _ => None
        };

        let mut result = format!("{components} <{token_type_display}>");

        if let Some(literal) = literal {
            result = format!("{result} '{}'", literal.blue().bold());
        }

        $(
            result = format!("{result} {}", $extra.to_string().purple());
        )?

        result
    }};
    (
        $precesor:ident ; $($segment:ident@$block:ident)|* <$($token_type:ident):+> $($extra:expr)?
    ) => {{
        use colored::Colorize;
        format!("{} {}", stringify!($precesor;).green().bold(), expected_token!($($segment@$block)|* <$($token_type):+> $($extra)?))
    }};
}

pub(crate) use identify_token;