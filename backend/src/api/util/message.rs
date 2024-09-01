use lazy_static::lazy_static;
use std::collections::HashMap;

// サーバーメッセージ
lazy_static! {
    pub static ref SVR_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("SVR_BUILD_SUCCESS_MSG", "🚀 サーバー構築に成功しました");
        map.insert("SVR_BUILD_FAILURE_MSG", "🔥 サーバー構築に失敗しました。");
        map.insert("TOKEN_NOT_FOUND_IN_REQUEST_HEADER_MSG", "リクエストヘッダーに認証トークンが見つかりませんでした。");

        map
    };
}

// DBメッセージ
lazy_static! {
    pub static ref DB_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("DB_CONNECTION_SUCCESS_MSG", "✅ データベース接続に成功しました。");
        map.insert("DB_CONNECTION_FAILURE_MSG", "🔥 データベース接続に失敗しました。");
        map.insert("TRANSACTION_COMMIT_FAILURE_MSG", "トランザクションをコミット中にエラーが発生しました：");
        map.insert("TRANSACTION_ROLLBACK_FAILURE_MSG", "トランザクションをロールバック中にエラーが発生しました：");
        map.insert("USER_INFO_NOT_FOUND_MSG", "ユーザー情報が存在しません。");

        map.insert("CREATE_DATA_SUCCESS_MSG", "データ作成に成功しました。");

        map
    };
}

// 環境変数の未設定メッセージ
lazy_static! {
    pub static ref SET_ENV_MSG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("NO_SET_ENV_VAR_FRONTEND_PORT", "⚠️ 環境変数が設定されていません: FRONTEND_PORT");
        map.insert("DATABASE_URL", "⚠️ 環境変数が設定されていません: DATABASE_URL");

        map
    };
}

// // SQLメッセージ
// lazy_static! {
//     pub static ref SQL_MSG: HashMap<&'static str, &'static str> = {
//         let mut map = HashMap::new();
//         map.insert("NO_SET_ENV_VAR_FRONTEND_PORT", "⚠️ 環境変数が設定されていません: FRONTEND_PORT");
//         map.insert("DATABASE_URL", "⚠️ 環境変数が設定されていません: DATABASE_URL");
//         map
//     };
// }