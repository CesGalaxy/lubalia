use clap::Parser;

pub mod run;

pub trait LubuggerCommand: Parser {
    fn run(&self);
}
