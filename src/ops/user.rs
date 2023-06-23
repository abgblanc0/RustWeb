use crate::db::establish_connection;
use crate::models::user::{NewUser, User};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::hyper::server::conn::Connection;
use rocket::serde::json::Json;

pub fn create_user(new_user: NewUser) {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving user");
}
pub fn get_users() -> Json<Vec<User>> {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let results = users.load::<User>(&mut connection).unwrap();

    Json(results)
}

pub fn get_user(name: &str) -> Option<User> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    match users
        .filter(username.eq(name))
        .first::<User>(&mut connection)
    {
        Ok(user) => Some(user),
        Err(error) => None,
    }
}

pub fn veryfy_user(input_username: &str, input_password: &str) -> bool {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    let result = users
        .filter(username.eq(input_username))
        .filter(password.eq(input_password))
        .first::<User>(&mut connection);

    result.is_ok()
}
