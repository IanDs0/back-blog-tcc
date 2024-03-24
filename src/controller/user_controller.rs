use rocket::Route;
use uuid::Uuid;

use rocket::serde::json::Json;

use crate::shared::dtos::user::CreateUserDto;
use crate::shared::dtos::user::CreatedUserDto;
use crate::services::user::find_user_by_id;
use crate::services::user::get_all_users;
use crate::services::user::create_user;

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


#[derive(Clone)]
pub struct UserController{
    pub routes: Vec<Route>,
}
impl UserController {
    pub fn new() -> Self {
        let routes = routes![
            get_all, 
            find_by_id, 
            create
        ];
        UserController {
            routes,
        }
    }
}