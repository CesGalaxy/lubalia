use lubalang::{lexer::lexer, parser::parser};

pub fn eval(source_code: String) {
    let lexer_result = lexer(source_code);

    match lexer_result {
        Ok(tokens) => {
            let parser_result = parser(tokens);

            match parser_result {
                Ok(ast) => {
                    println!("AST: {:?}", ast);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}