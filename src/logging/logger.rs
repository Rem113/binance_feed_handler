use std::io::Error;

use crate::logging::log_writer::LogWriter;

pub struct Logger<W: LogWriter> {
    writer: W,
}

impl<W: LogWriter> Logger<W> {
    pub fn new(writer: W) -> Logger<W> {
        Logger { writer }
    }

    pub fn log(&mut self, message: &str) -> Result<(), Error> {
        self.writer.write(message)
    }
}