// LRE -> Lubalia Runtime Environment

pub mod vfs;

use lubengine::run;
use vfs::LREVFS;

fn main() {
    println!("Welcome to Lubalia!");

    if cfg!(windows) {
        colored::control::set_override(true);
    }

    let args: Vec<_> = std::env::args().collect();

    let file = args.get(1).expect("No file provided!").clone();

    // Read test file
    test_file(file);
}

/// Get a file from storage, parse it and run it in the VM
fn test_file(file_name: String) {
    let vfs = LREVFS::new();

    let result = run(&vfs, file_name);

    println!("{:?}", result);
}
