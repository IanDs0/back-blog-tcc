use rocket::http::Status;

use uuid::Uuid;

pub fn delete_user(id: Uuid) -> (Status, &'static str) {
    (Status::new(200), "User {id} deleted") // Removed the semicolon at the end
}