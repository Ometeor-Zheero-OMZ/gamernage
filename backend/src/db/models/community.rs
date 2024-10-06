use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub profile_image: String,
    pub bio: String,
}
