use self::models::*;
use diesel::prelude::*;
use database::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut database::establish_connection();
    let results = users
        // .filter(level.eq(5))
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading user");

    println!("Displaying {} user", results.len());
    println!("-----------");

    for result in results {
        println!("{}", result.user_tag);
        println!("{}", result.email);
        println!("-----------");
    }
}
