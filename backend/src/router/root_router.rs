use crate::activitypub;
use crate::api_ext;
use crate::api_v1;
use crate::api_v2;
use crate::attachments;
use crate::nodeinfo;
use crate::oauth;
use crate::well_known;
use actix_web::web;

pub fn root_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/nodeinfo/2.0", web::get().to(nodeinfo::index))
        .service(
            web::scope("/api/v1")
                .route("/instance", web::get().to(api_v1::instance::get))
                .route("/statuses", web::post().to(api_v1::statuses::post))
                .service(api_v1::accounts::router::router())
                .route("/apps", web::post().to(api_v1::apps::post))
                .route(
                    "/apps/verify_credentials",
                    web::get().to(api_v1::apps::verify_credentials),
                )
                .service(api_v1::statuses::router())
                .route("/timelines/home", web::get().to(api_v1::timelines::home))
                .route("/favourites", web::get().to(api_v1::favourites::get)),
        )
        .service(api_v2::router())
        .service(web::scope("/api/ext").route("/login", web::post().to(api_ext::login::post)))
        .service(oauth::router())
        .route(
            "/system/media_attachments/files/{key}",
            web::get().to(attachments::get),
        )
        .service(
            web::scope("/.well-known")
                .route("/host-meta", web::get().to(well_known::host_meta))
                .route("/nodeinfo", web::get().to(well_known::nodeinfo))
                .route("/webfinger", web::get().to(well_known::webfinger)),
        )
        .route("/inbox", web::post().to(activitypub::inbox))
        .service(activitypub::router::users());
}
