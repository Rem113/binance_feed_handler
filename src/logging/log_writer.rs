use std::io::Error;

pub trait LogWriter {
    fn write(&mut self, log: &str) -> Result<(), Error>;
}