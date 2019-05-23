#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_more;

use actix_web::{web, App, HttpServer};

mod models;
mod route_handlers;
mod routes;
mod schema;
mod utils;

fn main() -> std::io::Result<()> {
    let addr = std::env::var("APP_ADDRESS").expect("APP_ADDRESS env not set");
    let port = std::env::var("APP_PORT").expect("APP_PORT env not set");
    let bind_addr = format!("{}:{}", addr, port);

    let server = HttpServer::new(|| {
        App::new()
            .configure(crate::routes::config)
            .default_service(web::route().to(crate::route_handlers::route_not_found))
    });

    server
        .bind(&bind_addr)
        .unwrap_or_else(|_| panic!("Could not bind app on {}", &bind_addr))
        .run()
}
