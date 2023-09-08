use crate::storage::LocalStorage;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize)]
pub struct AttachmentsGetRequest {
    key: String,
}

pub async fn get(
    req: web::Path<AttachmentsGetRequest>,
    storage: web::Data<LocalStorage>,
) -> impl Responder {
    let key = req.key.clone();

    let mut file = match storage.get_file(&key) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let ext = match Path::new(&key).extension() {
        Some(ext) => ext.to_string_lossy().into_owned(),
        None => return HttpResponse::NotFound().finish(),
    };

    let content_type = match ext.as_str() {
        "png" => "image/png",
        "jpg" => "image/jpeg",
        _ => return HttpResponse::NotFound().finish(),
    };

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    HttpResponse::Ok().content_type(content_type).body(data)
}
