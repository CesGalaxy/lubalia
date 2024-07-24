use super::ProcessLogger;

pub struct LoggingService<LogData: std::fmt::Display> {
    pub loggers: Vec<ProcessLogger<LogData>>,
}

impl<LogData: std::fmt::Display> LoggingService<LogData> {
    pub fn new() -> Self {
        Self {
            loggers: vec![],
        }
    }

    pub fn add_logger(&mut self, process: String) -> &mut ProcessLogger<LogData> {
        let logger = ProcessLogger::new(process);
        self.loggers.push(logger);
        self.loggers.last_mut().expect("logger was just added so it should be the las one")
    }
}