//! User Model

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
   pub id: i32,
   pub name: String,
   pub email: String,
   pub token: String
}
