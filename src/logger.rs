use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use crossterm::style::{StyledContent, Stylize};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct Logger {
    log_level: LogLevel,
    sender: Sender<String>,
}

impl Logger
{
    pub fn new<T>(log_level: LogLevel, log_file: T) -> Logger
        where T: Write + Send + 'static
    {
        let (sender, receiver): (Sender<String>, Receiver<String>) = channel();

        let log_file = Mutex::new(Box::new(log_file) as Box<dyn Write + Send>);

        thread::spawn(move || {
            while let Ok(log_line) = receiver.recv() {
                let mut log_file = log_file.lock().unwrap();
                log_file.write_all(log_line.as_bytes()).unwrap();
                log_file.flush().unwrap();
            }
        });

        Logger {
            sender,
            log_level,
        }
    }

    pub fn log<D: Display>(&self, log_level: LogLevel, message: D)
    {
        if log_level > self.log_level {
            return;
        }

        self.sender.send(message.to_string()).unwrap();
    }

    pub fn error(&self, message: &str) {
        let message = format!("[{:?}] {}\n", LogLevel::Error, message);
        self.log(LogLevel::Error, message.red());
    }

    pub fn warning(&self, message: &str) {
        let message = format!("[{:?}] {}\n", LogLevel::Warning, message);
        self.log(LogLevel::Warning, message.yellow());
    }

    pub fn info(&self, message: &str) {
        let message = format!("[{:?}] {}\n", LogLevel::Info, message);
        self.log(LogLevel::Info, message.green());
    }

    pub fn debug(&self, message: &str) {
        let message = format!("[{:?}] {}\n", LogLevel::Debug, message);
        self.log(LogLevel::Debug, message.blue());
    }

}
