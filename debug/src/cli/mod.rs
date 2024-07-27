pub mod commands;

use clap::Parser;
use self::commands::LubuggerCommand;

/// A debugger for the Lubalia Programming Language
#[derive(Parser, Debug)]
#[command(name = "lubugger", bin_name = "lubugger", version, about, long_about = None)]
pub enum LubuggerCli {
    Run(commands::run::RunnerCommand)
}

impl LubuggerCommand for LubuggerCli {
    fn run(&self) {
        match self {
            LubuggerCli::Run(run) => run.run()
        }
    }
}
