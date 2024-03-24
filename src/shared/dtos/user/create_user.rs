use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::enum_type::user::Level;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateUserDto {
    pub id: Option<Uuid>,
    pub user_tag: String,
    pub level: Option<Level>,
    pub email: String,
    pub password: String,
}
