use std::fs::File;
use std::io::{Error, Write};

use crate::logging::log_writer::LogWriter;

pub struct FileLogWriter {
    file: File,
}

impl FileLogWriter {
    pub fn new(filename: &str) -> Result<FileLogWriter, Error> {
        let file = std::fs::File::create(filename)?;
        Ok(FileLogWriter { file })
    }
}

impl LogWriter for FileLogWriter {
    fn write(&mut self, log: &str) -> Result<(), Error> {
        self.file.write(format!("{}\n", log).as_bytes())?;
        Ok(())
    }
}