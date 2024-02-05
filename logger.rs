// contains the logger for the application, which supports
// log levels and colored output.

pub enum Level {
    Info,
    Warn,
    Error,
}

pub struct Logger {}

impl Logger {
    fn log(level: Level, message: &str) {
        match level {
            Level::Info => println!("[INFO] {}", message),
            Level::Warn => println!("\x1b[33m[WARN] {}\x1b[0m", message),
            Level::Error => println!("\x1b[31m[ERR] {}\x1b[0m", message),
        }
    }

    pub fn info(message: &str) {
        Self::log(Level::Info, message);
    }

    pub fn warn(message: &str) {
        Self::log(Level::Warn, message);
    }

    pub fn error(message: &str) {
        Self::log(Level::Error, message);
    }
}
