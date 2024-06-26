use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{core::enum_type::user::Level, shared::dtos::user::{CreateUserDto, CreatedUserDto}};

pub fn create_user(user: Json<CreateUserDto>) -> Json<CreatedUserDto> {
    let response: Json<CreatedUserDto> = Json(
        CreatedUserDto {
            id: user.id.unwrap_or_else(|| Uuid::new_v4()).clone(),
            user_tag: user.user_tag.clone(),
            level: user.level.as_ref().unwrap_or_else(|| &Level::Free).clone(),
            email: user.email.clone(),
            password: None,
        }
    );
    response
}