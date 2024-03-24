use crate::core::enum_type::user::Level;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Uuid,
    pub user_tag: String,
    pub level: Level,
    pub email: String,
    pub password: String,
}
