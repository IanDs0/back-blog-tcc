// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewUser, User};

pub fn create_user(conn: &mut PgConnection, id: &str, user_tag: &str, level: i32, email: &str, password: &str) -> User {
    use crate::schema::users;

    let new_post = NewUser { id, user_tag, level, email, password };

    diesel::insert_into(users::table)
        .values(&new_post)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}
