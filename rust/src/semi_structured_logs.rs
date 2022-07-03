use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{level}]: {message}")
}

pub fn info(message: &str) -> String {
    format!("[INFO]: {message}")
}

pub fn warn(message: &str) -> String {
    format!("[WARNING]: {message}")
}

pub fn error(message: &str) -> String {
    format!("[ERROR]: {message}")
}
