#![allow(unused)]
#[macro_use]
extern crate rocket;

mod cors;
mod db;
mod models;
mod ops;
mod schema;
mod static_resources;

use chrono::format::format;
use models::user::{User, UserLogin};
use rocket::form::Form;
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::http::ContentType;
use rocket::local::asynchronous;
use rocket::response::status::NotFound;
use rocket::response::{content, Redirect};
use rocket::serde::json::Json;
use rocket::Data;
use rocket_dyn_templates::{context, Template};
use static_resources::StaticFile;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/users/<username>")]
fn get_user(username: String) -> String {
    //TODO
    format!("Hello, {}", username)
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    ops::user::get_users()
}

#[post("/login", data = "<user_data>")]
fn login(user_data: Form<UserLogin>) -> Result<Redirect, String> {
    //TODO
    if ops::user::veryfy_user(&user_data.username, &user_data.password) {
        let uri = format!("/users/{}", user_data.username);
        Ok(Redirect::to(uri))
    } else {
        Err(String::from("No user or password correct"))
    }
}

#[get("/<file..>")]
async fn static_file(file: PathBuf) -> Result<StaticFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    Ok(StaticFile(
        NamedFile::open(&path)
            .await
            .map_err(|err| NotFound(err.to_string()))?,
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![index, login])
        .mount("/static", routes![static_file])
        .mount("/api", routes![get_user, get_users])
        .attach(Template::fairing())
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}
