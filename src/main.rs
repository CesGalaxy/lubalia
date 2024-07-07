use app::{lang::{lexer::lexer, parser::parse_tree}, vm::VM};

fn main() {
    println!("Hello, world!");

    test_file("examples/variables.lub");
}

fn test_file(file_name: &str) {
    let file = std::fs::read_to_string(file_name);
    
    if let Ok(code) = file {
        let tokenizer_result = lexer(code);


        if let Ok(tokens) = tokenizer_result {
            for token in &tokens {
                print!("{token} ");
            }

            println!("({})", tokens.len());

            let tree = parse_tree(tokens);

            println!("TREE: {:?}", tree);

            if let Ok(program) = tree {
                let mut vm = VM::new(program);
                vm.run();
            }
        }
    }
}
