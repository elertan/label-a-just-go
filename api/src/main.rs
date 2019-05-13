#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket_contrib::json::JsonValue;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use crate::models::api_result::{ApiResult, ApiError, ApiErrorCode};
use std::sync::Mutex;

mod routes;
mod models;
mod extensions;
mod utils;
mod schema;

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    ApiResult::err(ApiError {
        err_code: ApiErrorCode::NotFound,
        message: "The requested route was not found".to_string()
    }).to_json()
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn setup() {
    // Check if app directories exist
    if !std::path::Path::new("./temp").exists() {
        std::fs::create_dir("./temp").expect("Could not create temp directory");
    }
}

pub struct RocketState {
    conn: Mutex<PgConnection>,
}

fn main() {
    setup();
    let db_conn = establish_connection();

    rocket::ignite()
        .manage(RocketState { conn: Mutex::new(db_conn) })
        .mount("/", rocket_codegen::routes![
            routes::ping,
            routes::api::v1::registration_details,
            routes::api::v1::extract_face
        ])
        .register(rocket_codegen::catchers![not_found])
        .launch();
}
