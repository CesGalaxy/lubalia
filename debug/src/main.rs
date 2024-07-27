extern crate clap;
extern crate iced;

// use iced::{Sandbox, Settings};
// use ui::DebuggerApp;

pub mod log;
pub mod ui;
pub mod vm;
pub mod cli;

use clap::Parser;
use cli::commands::LubuggerCommand;

pub fn main() -> Result<(), iced::Error> {
    let cmd = cli::LubuggerCli::parse();

    println!("{:?}", cmd);

    cmd.run();

    Ok(())

    //DebuggerApp::run(Settings::default())
}