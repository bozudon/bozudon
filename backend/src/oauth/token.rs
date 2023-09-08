use super::utils::is_acceptable_scope;
use crate::model::prelude::*;
use crate::randstr;
use crate::{entity, model::*};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{entity::*, ActiveModelTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenGetRequest {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
}

pub async fn post(
    req: web::Json<TokenGetRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    if req.grant_type != "authorization_code" {
        return HttpResponse::BadRequest().finish();
    }

    let (grant, app) = match OauthAccessGrant::find()
        .find_also_related(App)
        .filter(oauth_access_grant::Column::Token.eq(req.code.clone()))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some((grant, app)) => (grant, app.unwrap()),
        None => return HttpResponse::Unauthorized().finish(),
    };

    if req.client_id != app.client_id
        || req.client_secret != app.client_secret
        || grant.resource_owner_id.is_none()
    {
        return HttpResponse::Unauthorized().finish();
    }

    if req.redirect_uri != app.redirect_uri
        || (req.scope.is_some() && !is_acceptable_scope(&grant.scopes, &req.scope.clone().unwrap()))
    {
        return HttpResponse::BadRequest().finish();
    }

    let access_token = oauth_access_token::ActiveModel {
        id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
        token: Set(String::from(randstr::gen(Some(String::from("bdt"))))),
        app_id: Set(app.id),
        resource_owner_id: Set(grant.resource_owner_id.unwrap()),
        scopes: Set(req.scope.clone().unwrap_or(grant.scopes)),
    };

    let access_token = access_token.insert(db.as_ref()).await.unwrap();

    HttpResponse::Ok().json(entity::Token {
        access_token: access_token.token,
        token_type: String::from("Bearer"),
        scope: String::from(access_token.scopes),
        created_at: access_token.created_at.timestamp(),
    })
}
