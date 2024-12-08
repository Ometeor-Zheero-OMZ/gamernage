//! # カスタムロガー
//! 
//! ヘッダーにステータスを付与してログを出力
//! 
//! ## 関数
//! 
//! - `log`: ログ出力
//! 
//! ## マクロ
//! 
//! - `app_log`:     以下ログを出力するための土台　※直接的には使用しない
//! - `success_log`: 成功ログ
//! - `info_log`:    情報ログ
//! - `warning_log`: 警告ログ
//! - `error_log`:   エラーログ

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use lazy_static::lazy_static;

use crate::PROJECT_PATH;

lazy_static! {
    pub static ref LOG_PATH: String = format!("{}/log/actix.log", PROJECT_PATH);
}

/// Header 列挙子
#[allow(dead_code)]
pub enum Header {
    SUCCESS,
    INFO,
    WARNING,
    ERROR
}

/// log 関数
/// 
/// # 引数
/// 
/// * `header`  - `Header` 列挙子
/// * `message` - `&str`   出力するメッセージ
pub fn log(header: Header, message: &str) {
    let header = match header {
        Header::SUCCESS => "SUCCESS",
        Header::INFO => "INFO",
        Header::WARNING => "WARNING",
        Header::ERROR => "ERROR"
    };

    println!("[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message);

    if Path::new(&*LOG_PATH).exists() {
        // ログファイルが存在する場合は、そのファイルにログを出力
        let mut log_file = OpenOptions::new().append(true).open(&*LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    } else {
        // ログファイルが存在しない場合は、ファイルを生成しログを出力
        let mut log_file = OpenOptions::new().create_new(true).append(true).open(&*LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    }
}

#[macro_export]
macro_rules! app_log {
    // 引数あり
    ($header:expr, $msg:expr, $($arg:tt)*) => {
        let formatted_message = format!($msg, $($arg)*);
        crate::application::helpers::logger::log($header, &formatted_message);
    };
    // 引数なし
    ($header:expr, $msg:expr) => {
        crate::application::helpers::logger::log($header, $msg);
    };
}

#[macro_export]
macro_rules! success_log {
    // 引数あり
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::application::helpers::logger::Header::SUCCESS, $msg, $($arg)*);
    };
    // 引数なし
    ($msg:expr) => {
        app_log!(crate::application::helpers::logger::Header::SUCCESS, $msg);
    }
}

#[macro_export]
macro_rules! info_log {
    // 引数あり
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::application::helpers::logger::Header::INFO, $msg, $($arg)*);
    };
    // 引数なし
    ($msg:expr) => {
        app_log!(crate::application::helpers::logger::Header::INFO, $msg);
    };
}

#[macro_export]
macro_rules! warning_log {
    // 引数あり
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::application::helpers::logger::Header::WARNING, $msg, $($arg)*);
    };
    // 引数なし
    ($msg:expr) => {
        app_log!(crate::application::helpers::logger::Header::WARNING, $msg);
    };
}

#[macro_export]
macro_rules! error_log {
    // 引数あり
    ($msg:expr, $($arg:tt)*) => {
        app_log!(crate::application::helpers::logger::Header::ERROR, $msg, $($arg)*);
    };
    // 引数なし
    ($msg:expr) => {
        app_log!(crate::application::helpers::logger::Header::ERROR, $msg);
    };
}