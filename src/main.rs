use app::{lang::{lexer::lexer, parser::parse_tree}, vm::VM};

fn main() {
    println!("Hello, world!");

    // Read test file
    test_file("examples/variables.lub");
}

/// Get a file from storage, parse it and run it in the VM
fn test_file(file_name: &str) {
    let file = std::fs::read_to_string(file_name);
    
    if let Ok(code) = file {
        let tokenizer_result = lexer(code);

        if let Ok(tokens) = tokenizer_result {
            // Print the tokens lexed
            for token in &tokens {
                print!("{token} ");
            }

            println!("({})", tokens.len());

            // Parse the tokens and print the AST
            let tree = parse_tree(tokens);

            println!("TREE: {:?}", tree);

            if let Ok(program) = tree {
                // Create and run the VM
                let mut vm = VM::new(program);
                vm.run();
            }
        }
    }
}
