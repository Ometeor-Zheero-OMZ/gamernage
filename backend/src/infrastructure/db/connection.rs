//! DBプール設定
//! 
//! DB接続に必要な値を環境変数として取得し、Config に設定
//! 
//! # 関数
//! 
//! * `get_config` - `tokio_postgres::Config` を作成し、各環境変数を設定

use std::time::Duration;
use std::env;
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use tokio_postgres::NoTls;

use crate::infrastructure::config::db_config::get_config;

pub async fn get_db_pool () -> Pool<PostgresConnectionManager<NoTls>> {
    let max_pool_size: u32 = env::var("DATABASE_MAX_POOL_SIZE")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .expect("環境変数 `DATABASE_MAX_POOL_SIZE` は正しい整数値で設定する必要があります。");

    let min_idle_connect: u32 = env::var("MIN_IDLE_CONNECTION")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("環境変数 `MIN_IDLE_CONNECTION` は正しい整数値で設定する必要があります。");

    let connect_timeout: u64 = env::var("DATABASE_CONNECT_TIMEOUT")
        .unwrap_or_else(|_| "60".to_string()) 
        .parse()
        .expect("環境変数 `DATABASE_CONNECT_TIMEOUT` は正しい整数値で設定する必要があります。");

    // DB接続設定
    let config = get_config();
    let pg_mgr = PostgresConnectionManager::new(config, NoTls);

    // 接続プールの作成
    let pool = Pool::builder()
        .max_size(max_pool_size) // 最大接続数
        .min_idle(Some(min_idle_connect)) // 最小アイドル接続
        .idle_timeout(Some(Duration::from_secs(connect_timeout))) // 指定した時間を超えると接続を開放
        .build(pg_mgr)
        .await
        .expect("DB接続プールの作成に失敗しました");

    pool
}