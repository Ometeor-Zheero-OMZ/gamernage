use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use lazy_static::lazy_static;

// ログファイルのパス
use crate::PROJECT_PATH;

lazy_static! {
    pub static ref LOG_PATH: String = format!("{}/log/actix.log", PROJECT_PATH);
}

/// ログヘッダーの列挙子
#[allow(dead_code)]
pub enum Header {
    SUCCESS,
    INFO,
    WARNING,
    ERROR
}

/// ログを出力
pub fn log(header: Header, message: &str) {
    // ログヘッダーを判定
    let header = match header {
        Header::SUCCESS => "SUCCESS",
        Header::INFO => "INFO",
        Header::WARNING => "WARNING",
        Header::ERROR => "ERROR"
    };

    // ログをコンソール上に出力
    println!("[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message);

    // ログをファイルに書き出し
    if Path::new(&*LOG_PATH).exists() {
        let mut log_file = OpenOptions::new().append(true).open(&*LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    } else {
        let mut log_file = OpenOptions::new().create_new(true).append(true).open(&*LOG_PATH).unwrap();
        writeln!(log_file, "[{}] {} {}", Local::now().format("%m-%d-%Y %H:%M:%S").to_string(), header, message).unwrap();
    }
}
