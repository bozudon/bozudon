use crate::api_v1::utils::*;
use crate::entity;
use crate::model::*;
use crate::{config::Config, storage::LocalStorage};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::DatabaseConnection;
use sea_orm::*;

#[derive(MultipartForm)]
pub struct PostRequest {
    file: TempFile,
    description: Option<Text<String>>,
}

pub async fn post(
    auth: Option<BearerAuth>,
    req: MultipartForm<PostRequest>,
    config: web::Data<Config>,
    storage: web::Data<LocalStorage>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let user_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let description = match req.description {
        Some(ref description) => description.0.to_string(),
        None => String::from(""),
    };

    let content_type = match req.file.content_type {
        Some(ref content_type) => content_type.to_owned(),
        None => return HttpResponse::BadRequest().body("content-type required for file."),
    };

    let saved_media = match storage.save_file(&mut req.file.file.as_file(), content_type.clone()) {
        Ok(saved_media) => saved_media,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let media = media::ActiveModel {
        id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
        key: Set(saved_media.key.clone()),
        preview_key: Set(saved_media.thumbnail_key.clone()),
        media_type: Set(content_type.to_string()),
        description: Set(description.clone()),
        blurhash: NotSet,
        resource_owner_id: Set(Some(user_id)),
        remote_url: NotSet,
    };
    let media = match media.save(db.as_ref()).await {
        Ok(media) => media,
        Err(err) => {
            log::info!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(entity::MediaAttachment {
        id: media.id.unwrap().to_string(),
        media_type: saved_media.media_type,
        url: format!(
            "{}/system/media_attachments/files/{}",
            config.server_url, saved_media.key
        ),
        preview_url: format!(
            "{}/system/media_attachmetns/files/{}",
            config.server_url, saved_media.thumbnail_key
        ),
        remote_url: None,
        text_url: None,
        meta: (),
        description,
        blurhash: None,
    })
}
