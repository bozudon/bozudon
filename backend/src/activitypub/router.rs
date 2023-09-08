use crate::activitypub;
use actix_web::{web, Scope};

pub fn users() -> Scope {
    web::scope("/users/{name}")
        .route("", web::get().to(activitypub::user))
        .route("/inbox", web::post().to(activitypub::inbox))
}
