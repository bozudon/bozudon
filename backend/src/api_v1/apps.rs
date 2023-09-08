use crate::api_v1::utils::*;
use crate::entity;
use crate::model::*;
use crate::randstr;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct AppsPostRequest {
    pub client_name: String,
    pub redirect_uris: String,
    pub scopes: Option<String>,
    pub website: Option<String>,
}

fn is_valid_scopes(scopes: &str) -> bool {
    scopes
        .split(' ')
        .all(|e| e == "read" || e == "write" || e == "follow" || e == "push")
}

pub async fn post(
    req: web::Form<AppsPostRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    if req.scopes.is_some() {
        if !is_valid_scopes(&req.scopes.clone().unwrap()) {
            return HttpResponse::BadRequest().finish();
        }
    }

    let app = app::ActiveModel {
        id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
        name: Set(req.client_name.clone()),
        client_id: Set(randstr::gen(None)),
        client_secret: Set(randstr::gen(Some(String::from("bds")))),
        redirect_uri: Set(req.redirect_uris.clone()),
        scopes: Set(req.scopes.clone().unwrap_or(String::from("read"))),
        website: Set(req.website.clone()),
    };
    let status = app.insert(db.as_ref()).await.unwrap();

    HttpResponse::Ok().json(entity::Application {
        id: status.id.to_string(),
        website: status.website,
        redirect_uri: status.redirect_uri,
        name: status.name,
        client_id: status.client_id,
        client_secret: status.client_secret,
    })
}

pub async fn verify_credentials(
    auth: Option<BearerAuth>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    match get_authenticated_app(auth, db.as_ref()).await {
        Some(app) => HttpResponse::Ok().json(entity::ApplicationWithoutClientInfo {
            name: app.name,
            website: app.website,
        }),
        None => HttpResponse::Unauthorized().json(
            [("error", "The access token is invalid")]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_scopes_valid() {
        let test_cases = ["read write follow push", "read write", "read read"];
        for test_case in test_cases {
            assert!(is_valid_scopes(test_case));
        }
    }

    #[test]
    fn is_valid_scopes_invalid() {
        let test_cases = ["read write follow hoge"];
        for test_case in test_cases {
            assert!(!is_valid_scopes(test_case));
        }
    }
}
