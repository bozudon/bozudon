use crate::oauth::authorize;
use crate::oauth::token;
use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("/oauth")
        .route("/authorize", web::get().to(authorize::get))
        .route("/authorize", web::post().to(authorize::post))
        .route("/token", web::post().to(token::post))
}
