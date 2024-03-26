use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub user_tag: String,
    pub level: i32,
    pub email: String,
    pub password: String,
}

use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub user_tag: &'a str,
    pub level: i32,
    pub email: &'a str,
    pub password: &'a str,
}