use std::fmt;

#[derive(Clone, PartialEq, Debug)]
enum LogLevel {
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

fn log(level: LogLevel, message: &str) -> String {
    format!("[{level}]: {message}")
}

fn info(message: &str) -> String {
    format!("[INFO]: {message}")
}

fn warn(message: &str) -> String {
    format!("[WARNING]: {message}")
}

fn error(message: &str) -> String {
    format!("[ERROR]: {message}")
}
