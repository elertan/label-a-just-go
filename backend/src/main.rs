use actix_web::{HttpServer, App, web, Responder};

fn registration_details_route(uuid_string: web::Path<(String)>) -> impl Responder {
    format!("Hello {}!", uuid_string)
}

fn main() -> std::io::Result<()> {
    let addr = std::env::var("APP_ADDRESS").expect("APP_ADDRESS env not set");
    let port = std::env::var("APP_PORT").expect("APP_PORT env not set");
    let bind_addr = format!("{}:{}", addr, port);

    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/api/v1/registration-details/{token}")
                    .route(web::get().to(registration_details_route))
            )
    });

    server.bind(&bind_addr)
        .expect(format!("Could not bind app on {}", &bind_addr).as_str())
        .run()
}
