#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;

use crate::db::DbExecutor;
use actix::prelude::*;
use actix_web::{middleware, web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};

mod db;
mod models;
mod repositories;
mod route_handlers;
mod routes;
mod schema;
mod utils;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn main() -> std::io::Result<()> {
    let addr = std::env::var("APP_ADDRESS").expect("APP_ADDRESS env not set");
    let port = std::env::var("APP_PORT").expect("APP_PORT env not set");
    let bind_addr = format!("{}:{}", addr, port);

    let server = HttpServer::new(|| {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

        App::new()
            .data(AppState { db: address })
            .wrap(middleware::Logger::default())
            .configure(crate::routes::config)
            .default_service(web::route().to(crate::route_handlers::route_not_found))
    });

    server
        .bind(&bind_addr)
        .unwrap_or_else(|_| panic!("Could not bind app on {}", &bind_addr))
        .run()
}
