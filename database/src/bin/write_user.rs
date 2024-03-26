use database::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut user_tag = String::new();
    let mut level = String::new();
    let mut email = String::new();
    let mut password = String::new();
    let mut id = String::new();

    println!("User_tag");
    stdin().read_line(&mut user_tag).unwrap();
    let user_tag = user_tag.trim_end(); // Remove the trailing newline

    println!("\nOk! id");
    stdin().read_line(&mut id).unwrap();
    let id = id.trim_end(); // Remove the trailing newline

    println!("\nOk! email\n");
    stdin().read_to_string(&mut email).unwrap();
    let email = email.trim_end(); // Remove the trailing newline

    println!("\nOk! password\n");
    stdin().read_to_string(&mut password).unwrap();
    let password = password.trim_end(); // Remove the trailing newline


    println!("\nOk! level\n");
    stdin().read_to_string(&mut level).unwrap();
    let level: i32 = match level.trim_end().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let user = create_user(connection, id, user_tag, level, email, password);
    // let post = create_user(connection, title, &body);
    println!("\nSaved draft {} with id {}", user_tag, user.id);
}
