#[macro_use] extern crate rocket;

use std::env;
use dotenvy::dotenv;

mod core;
mod shared;
mod services;
mod controller;
use controller::user_controller;


#[get("/")]
fn index() -> String {
    env::var("JWT_SECRET").unwrap()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .mount("/user", user_controller::UserController::new().routes)
}
