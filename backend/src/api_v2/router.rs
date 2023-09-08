use crate::api_v2::media;
use crate::api_v2::search;
use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("/api/v2")
        .route("/search", web::get().to(search::get))
        .route("/media", web::post().to(media::post))
}
