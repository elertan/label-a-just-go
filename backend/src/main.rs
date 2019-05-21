#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{HttpServer, App, web};

mod models;
mod schema;
mod utils;
mod routes;
mod route_handlers;

fn main() -> std::io::Result<()> {
    let addr = std::env::var("APP_ADDRESS").expect("APP_ADDRESS env not set");
    let port = std::env::var("APP_PORT").expect("APP_PORT env not set");
    let bind_addr = format!("{}:{}", addr, port);

    let server = HttpServer::new(|| {
        App::new()
            .configure(crate::routes::config)
            .default_service(
                web::route().to(crate::route_handlers::route_not_found)
            )
    });

    server.bind(&bind_addr)
        .expect(format!("Could not bind app on {}", &bind_addr).as_str())
        .run()
}
