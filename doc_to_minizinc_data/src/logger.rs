use std::io::{self, Write};
use std::fs;
use std::path::PathBuf;

// Simple logging struct
pub struct LogWriter {
    file: fs::File,
}

impl LogWriter {
    pub fn new(path: &PathBuf) -> io::Result<Self> {
        let file = fs::File::create(path)?;
        Ok(LogWriter { file })
    }

    pub fn log(&mut self, message: &str) {
        // Write to stdout
        println!("{}", message);
        // Write to file
        if let Err(e) = writeln!(&mut self.file, "{}", message) {
            eprintln!("Error writing to log file: {}", e);
        }
    }

    pub fn debug_log(&mut self, message: &str) {
        // Only write to file
        if let Err(e) = writeln!(&mut self.file, "{}", message) {
            eprintln!("Error writing to log file: {}", e);
        }
    }
}
