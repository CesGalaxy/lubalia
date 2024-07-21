use app::{lang::{lexer::lexer, parser::parser}, vm::VM};

fn main() {
    println!("Hello, world!");

    if cfg!(windows) {
        colored::control::set_override(true);
    }

    // Read test file
    test_file("examples/variables.lub");
}

/// Get a file from storage, parse it and run it in the VM
fn test_file(file_name: &str) {
    let file = std::fs::read_to_string(file_name);

    if let Ok(code) = file {
        let tokenizer_result = lexer(code);

        match tokenizer_result {
            Ok(tokens) => {
                // Print the tokens lexed
                for token in &tokens {
                    print!("{token}");
                }
    
                println!(" {} tokens lexed!", tokens.len());
                
                let tree = parser(tokens);
    
                if let Ok(program) = tree {
                    let program: Vec<_> = program.units().into_iter().cloned().collect();

                    for astri in &program {
                        println!("\t{astri:?}");
                    }
    
                    // Create and run the VM
                    let mut vm = VM::new();

                    vm.evaluate(program);

                    println!("{}", vm.global);
                } else {
                    println!("TREE: {:?}", tree);
                }
            },
            Err(error) => panic!("LexerError:\n{error}")
        }
    }
}
