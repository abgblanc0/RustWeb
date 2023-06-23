use serde::Serialize;

use crate::schema::users;

#[derive(diesel::Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(FromForm)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(diesel::Queryable, Debug, diesel::AsChangeset, Serialize)]
pub struct User {
    pub id: i32,
    pub ctime: Option<chrono::NaiveDateTime>,
    pub username: String,
    pub password: String,
}
