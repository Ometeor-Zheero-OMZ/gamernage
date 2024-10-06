//! User Model

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
   pub id: i32,
   pub name: String,
   pub email: String,
   pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
   pub id: i32,
   pub name: String,
   pub username: String,
   pub profile_image: String,
}

