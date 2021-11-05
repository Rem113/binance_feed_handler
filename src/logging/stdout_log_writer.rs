use std::io::Error;
use crate::logging::log_writer::LogWriter;

pub struct StdoutLogWriter {}

impl LogWriter for StdoutLogWriter {
    fn write(&mut self, log: &str) -> Result<(), Error> {
        println!("{}", log);
        Ok(())
    }
}