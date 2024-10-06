//! This module provides logging functionality for the web service.
//! It supports logging messages to both the console and a file, with different
//! log levels including `SUCCESS`, `INFO`, `WARNING`, and `ERROR`. Each log entry
//! is timestamped and appended to a log file located in the project's log directory.
//!
//! # Overview
//!
//! The `log` function is used to output log messages with a specified log level. The log
//! messages are printed to the console and saved to a log file for persistent storage.
//! The file path for the log file is determined by the `LOG_PATH` constant, which is based
//! on the project's root directory.
//!
//! # Dependencies
//!
//! This module relies on the following crates:
//! - `chrono`: Used for handling date and time to provide accurate timestamps in log entries.
//! - `lazy_static`: Used to define the static `LOG_PATH` which specifies where the log file is located.
//!
//! # Log Levels
//!
//! The module supports four log levels:
//! - `SUCCESS`: Indicates a successful operation or event.
//! - `INFO`: Provides general information about the application's state or progress.
//! - `WARNING`: Signals a potential issue that may require attention.
//! - `ERROR`: Represents an error that occurred, which may need to be addressed.
//!
//! # Example
//!
//! To log a message, use the `log` function and specify the log level and message:
//!
//! ```rust
//! use your_crate::logger;
//!
//! // Log an informational message
//! logger::log(logger::Header::INFO, "This is an informational message.");
//!
//! // Log an error message
//! logger::log(logger::Header::ERROR, "This is an error message.");
//! ```

use chrono::Local;
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

// The root path of the project where log files will be saved.
use crate::PROJECT_PATH;

lazy_static! {
    /// `LOG_PATH` defines the path to the log file where all log entries will be saved.
    /// The log file is located in the `log/` directory under the project's root path.
    pub static ref LOG_PATH: String = format!("{}/log/actix.log", PROJECT_PATH);
}

/// Enum representing the various log headers (log levels) used in the log system.
#[allow(dead_code)]
pub enum Header {
    SUCCESS,
    INFO,
    WARNING,
    ERROR,
}

/// Outputs a log message to both the console and the log file.
///
/// # Arguments
///
/// * `header` - The log level (e.g., `SUCCESS`, `INFO`, `WARNING`, `ERROR`).
/// * `message` - The log message to be output.
///
/// This function writes a log entry with a timestamp and the specified log level to
/// the console and appends it to a log file. If the log file does not exist, it creates
/// the file.
///
/// # Example
///
/// ```rust
/// logger::log(logger::Header::INFO, &format!("[message: {}]", success_message));
/// logger::log(logger::Header::ERROR, &format!("[message: {}]", db_error));
/// ```
pub fn log(header: Header, message: &str) {
    let header = match header {
        Header::SUCCESS => "SUCCESS",
        Header::INFO => "INFO",
        Header::WARNING => "WARNING",
        Header::ERROR => "ERROR",
    };

    println!(
        "[{}] {} {}",
        Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
        header,
        message
    );

    if Path::new(&*LOG_PATH).exists() {
        // If the log file already exists, append the new log entry
        let mut log_file = OpenOptions::new().append(true).open(&*LOG_PATH).unwrap();
        writeln!(
            log_file,
            "[{}] {} {}",
            Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
            header,
            message
        )
        .unwrap();
    } else {
        // If the log file does not exist, create it and then append the log entry
        let mut log_file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(&*LOG_PATH)
            .unwrap();
        writeln!(
            log_file,
            "[{}] {} {}",
            Local::now().format("%m-%d-%Y %H:%M:%S").to_string(),
            header,
            message
        )
        .unwrap();
    }
}

#[macro_export]
macro_rules! app_log {
    // 引数あり
    ($header:expr, $msg:expr, $($arg:tt)*) => {
        let formatted_message = format!($msg, $($arg)*);
        crate::libraries::logger::log($header, &formatted_message);
    };
    // 引数なし
    ($header:expr, $msg:expr) => {
        crate::libraries::logger::log($header, $msg);
    };
}

#[macro_export]
macro_rules! success_log {
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::libraries::logger::Header::SUCCESS, $msg, $($arg)*);
    };
    ($msg:expr) => {
        app_log!(crate::libraries::logger::Header::SUCCESS, $msg);
    }
}

#[macro_export]
macro_rules! info_log {
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::libraries::logger::Header::INFO, $msg, $($arg)*);
    };
    ($msg:expr) => {
        app_log!(crate::libraries::logger::Header::INFO, $msg);
    };
}

#[macro_export]
macro_rules! warning_log {
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::libraries::logger::Header::WARNING, $msg, $($arg)*);
    };
    ($msg:expr) => {
        app_log!(crate::libraries::logger::Header::WARNING, $msg);
    };
}

#[macro_export]
macro_rules! error_log {
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::libraries::logger::Header::ERROR, $msg, $($arg)*);
    };
    ($msg:expr) => {
        app_log!(crate::libraries::logger::Header::ERROR, $msg);
    };
}
