use std::{sync::{Arc, Mutex}};
use chrono::Local;

#[cfg(target_os = "windows")]
use ansi_term::{Colour, enable_ansi_support};

const MODULE: &str = "Logger";

lazy_static::lazy_static! {
    static ref LOGGER: Arc<Mutex<Logger>> = Arc::new(Mutex::new(Logger::default()));
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum LogLevel {
    Error,
    Success,
    Warning,
    Info,
    Debug,
}

pub struct Logger {
    log_level: LogLevel,
    writer: Box<dyn std::io::Write + Send>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            writer: Box::new(std::io::stdout()),
        }
    }
}

#[allow(dead_code)]
impl Logger {
    pub fn init(output_file: &Option<String>) -> Result<(), String> {
        let mut logger = LOGGER.lock().unwrap();
        *logger = Self::default();

        #[cfg(target_os = "windows")]
        enable_ansi_support().unwrap_or_default();

        let writer: Box<dyn std::io::Write + Send> = match output_file {
            Some(path) => {
                if path == "" {
                    Box::new(std::io::stdout())
                } else {
                    Box::new(std::fs::OpenOptions::new().create(true).append(true).open(path).map_err(|e| e.to_string())?)
                }
            },
            None => Box::new(std::io::stdout()),
        };

        *logger = Self {
            log_level: LogLevel::Info,
            writer,
        };

        Ok(())
    }

    pub fn set_level(level: &str) {
        let log_level = match level.to_uppercase().as_str() {
            "SUCCESS" => LogLevel::Success,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARNING" => LogLevel::Warning,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::Info,
        };
        
        let mut logger = LOGGER.lock().unwrap();

        logger.output_log(LogLevel::Info, MODULE, &format!("Logger 日志级别设置为: {:?}", log_level));
        logger.log_level = log_level;
    }

    fn output_log(&mut self, level: LogLevel, module: &str, message: &str) {
        if level > self.log_level {
            return;
        }

        let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");

        #[cfg(not(target_os = "windows"))]
        let reset_code = "\x1b[0m";

        #[cfg(not(target_os = "windows"))]
        let (level_str, color) = match level {
            LogLevel::Success => ("SUCCESS", "\x1b[94m"),
            LogLevel::Error => ("ERROR", "\x1b[91m"),
            LogLevel::Warning => ("WARNING", "\x1b[33m"),
            LogLevel::Info => ("INFO", "\x1b[90m"),
            LogLevel::Debug => ("DEBUG", "\x1b[95m"),
        };

        #[cfg(not(target_os = "windows"))]
        let _ = writeln!(self.writer, "{}[{}] [ 模块: {} ] [ 时间: {} ] {}{}", color, level_str, module, datetime, message, reset_code);

        #[cfg(target_os = "windows")]
        let level_str = match level {
            LogLevel::Success => Colour::Green.paint("SUCCESS"),
            LogLevel::Error => Colour::Red.paint("ERROR"),
            LogLevel::Warning => Colour::Yellow.paint("WARNING"),
            LogLevel::Info => Colour::Green.paint("INFO"),
            LogLevel::Debug => Colour::Blue.paint("DEBUG"),
        };

        #[cfg(target_os = "windows")]
        let _ = writeln!(self.writer, "[{:?}] [{}] [{}] {}", level_str, module, datetime, message);
    }
}

#[allow(dead_code)]
pub fn success(msg: &str, module: &str) {
    LOGGER.lock().unwrap().output_log(LogLevel::Success, module, msg);
}

#[allow(dead_code)]
pub fn error(msg: &str, module: &str) {
    LOGGER.lock().unwrap().output_log(LogLevel::Error, module, msg);
}

#[allow(dead_code)]
pub fn warning(msg: &str, module: &str) {
    LOGGER.lock().unwrap().output_log(LogLevel::Warning, module, msg);
}

#[allow(dead_code)]
pub fn info(msg: &str, module: &str) {
    LOGGER.lock().unwrap().output_log(LogLevel::Info, module, msg);
}

#[allow(dead_code)]
pub fn debug(msg: &str, module: &str) {
    LOGGER.lock().unwrap().output_log(LogLevel::Debug, module, msg);
}

#[allow(dead_code)]
pub fn log(msg: &str, module: &str) {
    LOGGER.lock().unwrap().output_log(LogLevel::Info, module, msg);
}
