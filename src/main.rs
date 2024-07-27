use eval::evaluate_code;
use lubengine::vm::VM;

pub mod eval;

fn main() {
    println!("Welcome to Lubalia!");

    if cfg!(windows) {
        colored::control::set_override(true);
    }

    let args: Vec<_> = std::env::args().collect();

    let file = args.get(1).expect("No file provided!");

    // Read test file
    test_file(file);
}

/// Get a file from storage, parse it and run it in the VM
fn test_file(file_name: &str) {
    let file = std::fs::read_to_string(file_name);

    if let Ok(code) = file {
        let mut vm = VM::new();

        let result = evaluate_code(&mut vm, code);

        if let Err(e) = result {
            println!("Evaluation error: {:?}", e);
        } else {
            println!("Program evaluated and executed successfully!");
        }
    }
}
