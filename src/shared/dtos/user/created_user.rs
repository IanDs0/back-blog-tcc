use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::enum_type::user::Level;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatedUserDto {
    pub id: Uuid,
    pub user_tag: String,
    pub level: Level,
    pub email: String,
    pub password: String,
}
