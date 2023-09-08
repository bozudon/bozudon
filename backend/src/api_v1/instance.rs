use crate::config::Config;
use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct InstanceUrls {
    streaming_api: String,
}

#[derive(Serialize)]
struct Instance {
    uri: String,
    title: String,
    short_description: String,
    description: String,
    email: String,
    version: String,
    urls: InstanceUrls,
}

pub async fn get(config: web::Data<Config>) -> impl Responder {
    web::Json(Instance {
        uri: config.uri.clone(),
        title: config.title.clone(),
        short_description: config.short_description.clone(),
        description: config.description.clone(),
        email: config.email.clone(),
        version: config.version.clone(),
        urls: InstanceUrls {
            streaming_api: config.streaming_api.clone(),
        },
    })
}
