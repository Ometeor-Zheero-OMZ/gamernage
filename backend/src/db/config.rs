use std::env;

/// 認証済みのユーザーデータを返却
/// 
/// # 引数
/// 
/// # 戻り値
/// 
/// * `Config` - データベース設定情報
pub fn get_config () -> tokio_postgres::Config {
    let mut config = tokio_postgres::Config::new();
    config.host(&env::var("DATABASE_HOST").expect("環境変数 `DATABASE_HOST` は設定する必要があります。"));
    config.user(&env::var("DATABASE_USER").expect("環境変数 `DATABASE_USER` は設定する必要があります。"));
    config.password(&env::var("DATABASE_PASSWORD").expect("環境変数 `DATABASE_PASSWORD` は設定する必要があります。"));
    config.dbname(&env::var("DATABASE_NAME").expect("環境変数 `DATABASE_NAME` は設定する必要があります。"));

    config
}