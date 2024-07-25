pub mod service;

use std::fmt::Display;

pub struct ProcessLogger<LogData: std::fmt::Display> {
    pub process: String,
    pub logs: Vec<LogEntry<LogData>>,
    pub counter: usize,
}

pub enum LogEntry<LogData> {
    Counter(Box<LogEntry<LogData>>, usize),
    Checkpoint(&'static str),
    Info(LogData),
    Warning(LogData),
    Error(LogData),
}

impl<LogData: std::fmt::Display> ProcessLogger<LogData> {
    pub fn new(process: String) -> Self {
        Self {
            process,
            logs: vec![],
            counter: 0
        }
    }

    pub fn log(&mut self, log: LogEntry<LogData>) {
        self.logs.push(log);
    }

    pub fn log_counter(&mut self, log: LogEntry<LogData>) {
        self.logs.push(LogEntry::Counter(Box::new(log), self.counter));
    }

    pub fn reset_counter(&mut self) {
        self.counter = 0;
    }

    pub fn count(&mut self) {
        self.counter += 1;
    }
}

impl<LogData: Display> Display for ProcessLogger<LogData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Process: {}", self.process)?;
        for log in &self.logs {
            writeln!(f, "\t{}", log)?;
        }
        Ok(())
    }
}

impl<LogData> Display for LogEntry<LogData> where LogData: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogEntry::Counter(entry, count) => write!(f, "{} with count {}", entry, count),
            LogEntry::Checkpoint(name) => write!(f, "Checkpoint: {}", name),
            LogEntry::Info(data) => write!(f, "Info: {}", data),
            LogEntry::Warning(data) => write!(f, "Warning: {}", data),
            LogEntry::Error(data) => write!(f, "Error: {}", data),
        }
    }
    
}
