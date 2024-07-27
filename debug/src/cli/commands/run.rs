use clap::Parser;

use super::LubuggerCommand;

/// Execute a script with debugging tools
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RunnerCommand {
    /// Filename value
    #[arg(default_value_t = String::from("main.lub"))]
    file: String,

    /// Show a window with debugging tools & info
    #[arg(long, default_value_t = false)]
    enable_ui: bool,
}

impl LubuggerCommand for RunnerCommand {
    fn run(&self) {
        println!("Running script: {}", self.file);
        if self.enable_ui {
            println!("Enabling UI");
        }
    }
}