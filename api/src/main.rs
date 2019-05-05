#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket_contrib::json::JsonValue;
use crate::models::api_result::{ApiResult, ApiError, ApiErrorCode};

mod routes;
mod models;
mod extensions;

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
    ApiResult::err(ApiError {
        err_code: ApiErrorCode::NotFound,
        message: "The requested route was not found".to_string()
    }).to_json()
}

fn main() {
    rocket::ignite()
        .mount("/", rocket_codegen::routes![
            routes::ping,
            routes::api::v1::registration_details
        ])
        .register(rocket_codegen::catchers![not_found])
        .launch();
}
