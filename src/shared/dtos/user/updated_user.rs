use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::enum_type::user::Level;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdatedUserDto {
    pub id: Option<Uuid>,
    pub user_tag: Option<String>,
    pub level: Option<Level>,
    pub email: Option<String>,
    pub password: Option<String>,
}
