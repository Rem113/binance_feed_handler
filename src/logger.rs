use core::fmt;
use std::error::Error;

use serde::__private::Formatter;

#[derive(Debug)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Log {
    pub level: LogLevel,
    pub message: String,
}

pub trait LogWriter {
    fn write(&self, log: Log) -> Result<(), Box<dyn Error>>;
}

pub struct StdOutLogWriter;

impl LogWriter for StdOutLogWriter {
    fn write(&self, log: Log) -> Result<(), Box<dyn Error>> {
        println!("{}: {}", log.level, log.message);
        Ok(())
    }
}

pub struct Logger<W: LogWriter> {
    writer: W
}

impl<W: LogWriter> Logger<W> {
    pub fn new(writer: W) -> Logger<W> {
        Logger { writer }
    }

    pub fn trace(&self, message: String) -> Result<(), Box<dyn Error>> {
        self.writer.write(Log { level: LogLevel::TRACE, message })
    }

    pub fn debug(&self, message: String) -> Result<(), Box<dyn Error>> {
        self.writer.write(Log { level: LogLevel::DEBUG, message })
    }

    pub fn info(&self, message: String) -> Result<(), Box<dyn Error>> {
        self.writer.write(Log { level: LogLevel::INFO, message })
    }

    pub fn warn(&self, message: String) -> Result<(), Box<dyn Error>> {
        self.writer.write(Log { level: LogLevel::WARN, message })
    }

    pub fn error(&self, message: String) -> Result<(), Box<dyn Error>> {
        self.writer.write(Log { level: LogLevel::ERROR, message })
    }

    pub fn fatal(&self, message: String) -> Result<(), Box<dyn Error>> {
        self.writer.write(Log { level: LogLevel::FATAL, message })
    }
}