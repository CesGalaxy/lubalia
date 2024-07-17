use app::lang::{lexer::lexer, parser::parser};

fn main() {
    println!("Hello, world!");

    if cfg!(target_os = "windows") {
        colored::control::set_virtual_terminal(true).unwrap();
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
                        print!("{:?} ", astri);
                    }
    
                    println!("({})", program.len());
    
                    // Create and run the VM
                    //let mut vm = VM::new(program);
                    //vm.run();
                    //println!("{:?}", vm.global);
                } else {
                    println!("TREE: {:?}", tree);
                }
            },
            Err(error) => panic!("LexerError:\n{error}")
        }
    }
}
