use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct LogWriter {
    file: File,
}

impl LogWriter {
    pub fn new(path: &PathBuf) -> io::Result<LogWriter> {
        let file = OpenOptions::new()
            .create(true)
            
            .append(true)
            .open(path)?;
        Ok(LogWriter { file })
    }

    pub fn log(&mut self, message: &str) {
        if let Err(e) = writeln!(self.file, "[LOG] {message}") {
            eprintln!("Failed to write to log file: {e}");
        }
    }

    pub fn debug_log(&mut self, message: &str) {
        if let Err(e) = writeln!(self.file, "[DEBUG] {message}") {
            eprintln!("Failed to write to log file: {e}");
        }
    }
}