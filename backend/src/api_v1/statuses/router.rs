use crate::api_v1::statuses;
use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("/statuses")
        .route("", web::post().to(statuses::post))
        .service(
            web::scope("/{id}")
                .route("", web::get().to(statuses::get))
                .route("/context", web::get().to(statuses::context))
                .route("/favourite", web::post().to(statuses::favourite))
                .route("/unfavourite", web::post().to(statuses::unfavourite))
                .route("/reblog", web::post().to(statuses::reblog))
                .route("/unreblog", web::post().to(statuses::unreblog)),
        )
}
