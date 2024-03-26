use rocket::serde::json::Json;
use uuid::Uuid;

use crate::shared::dtos::user::{UpdatedUserDto, UpdateUserDto};

pub fn update_user(id: Uuid, user: Json<UpdateUserDto>) -> Json<UpdatedUserDto> {
    let response: Json<UpdatedUserDto> = Json(
        UpdatedUserDto {
            id: Some(id.clone()),
            user_tag: user.user_tag.clone(),
            level: user.level.clone(),
            email: user.email.clone(),
            password: None,
        }
    );
    response
}