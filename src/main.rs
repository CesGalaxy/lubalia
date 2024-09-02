// LRE -> Lubalia Runtime Environment

pub mod vfs;

use lubengine::eval;
use vfs::LREVFS;

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

    if let Ok(source_code) = file {
        eval(source_code);

        let vfs = LREVFS::new();
    } else {
        println!("Error reading file: {:?}", file.err());
    }
}
