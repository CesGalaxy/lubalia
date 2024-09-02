pub mod vfs;

use lubalang::{lexer::lexer, parser::parser};
use lubalia_compiler::compile;

pub fn eval(source_code: String) {
    let lexer_result = lexer(source_code);

    match lexer_result {
        Ok(tokens) => {
            let parser_result = parser(tokens);

            match parser_result {
                Ok(ast) => {
                    println!("AST: {:?}", ast);

                    let program = compile(ast);

                    println!("Program: {:?}", program);
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