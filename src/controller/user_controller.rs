use rocket::http::Status;
use rocket::Route;
use uuid::Uuid;

use rocket::serde::json::Json;

use crate::shared::dtos::user::CreateUserDto;
use crate::shared::dtos::user::CreatedUserDto;
use crate::shared::dtos::user::UpdateUserDto;
use crate::shared::dtos::user::UpdatedUserDto;

use crate::services::user::find_user_by_id;
use crate::services::user::get_all_users;
use crate::services::user::create_user;
use crate::services::user::update_user;
use crate::services::user::delete_user;

#[get("/")]
fn get_all() -> String {
    get_all_users()
}

#[get("/<id>")]
fn find_by_id(id: Uuid) -> String {
    find_user_by_id(id)
}

#[post("/", format = "json", data = "<user>")]
fn create(user: Json<CreateUserDto>) -> Json<CreatedUserDto> {
    create_user(user)
}

#[put("/<id>", format = "json", data = "<user>")]
fn update(id: Uuid, user: Json<UpdateUserDto>) -> Json<UpdatedUserDto> {
    update_user(id, user)
}

#[delete("/<id>")]
fn delete(id: Uuid) -> (Status, &'static str) {
    delete_user(id)
}

pub struct UserController{
    pub routes: Vec<Route>,
}
impl UserController {
    pub fn new() -> Self {
        let routes = routes![
            get_all, 
            find_by_id, 
            create,
            update,
            delete
        ];
        UserController {
            routes,
        }
    }
}