extern crate iced;
use iced::{Sandbox, Settings};
use ui::DebuggerApp;

mod log;
pub mod ui;
pub mod vm;

pub fn main() -> Result<(), iced::Error> {
    println!("Hello, world!");

    DebuggerApp::run(Settings::default())
}