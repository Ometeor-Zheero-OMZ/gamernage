use serde::{Serialize, Deserialize};

/// 認証ユーザー
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
   pub id: i32,
   pub name: String,
   pub email: String,
   pub token: String
}