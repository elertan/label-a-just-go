use actix_web::{web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/v1/event-invitation/{token}")
            .route(web::get().to(crate::route_handlers::event_invitation))
    );
//    cfg.service(
//        web::resource("/api/v1/extract-face")
//            .route(web::post().to(crate::route_handlers::extract_face))
//    );
}
