use std::fs::File;
use std::io::{Error, Write};

pub trait LogWriter {
    fn write(&mut self, log: &str) -> Result<(), Error>;
}

pub struct FileLogWriter {
    file: File
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

pub struct Logger<W: LogWriter> {
    writer: W
}

impl<W: LogWriter> Logger<W> {
    pub fn new(writer: W) -> Logger<W> {
        Logger { writer }
    }

    pub fn log(&mut self, message: &str) -> Result<(), Error> {
        self.writer.write(message)
    }
}