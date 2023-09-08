use crate::model::prelude::*;
use crate::model::*;
use crate::randstr;
use actix_web::{http::header, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{entity::*, ActiveModelTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Deserialize;

use super::utils::is_acceptable_scope;

#[derive(Deserialize)]
pub struct AuthorizeGetRequest {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub force_login: Option<String>,
    pub lang: Option<String>,
}

pub async fn get(
    req: web::Query<AuthorizeGetRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let app = match App::find()
        .filter(app::Column::ClientId.eq(req.client_id.clone()))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(app) => app,
        None => return HttpResponse::Unauthorized().body("client_id is not valid"),
    };

    if req.response_type != "code" {
        return HttpResponse::BadRequest().body("response_type must be 'code'");
    }
    if req.scope.is_some() && !is_acceptable_scope(&app.scopes, &req.scope.clone().unwrap()) {
        return HttpResponse::BadRequest().body("scope contains permissions not specified in app");
    }

    let grant = oauth_access_grant::ActiveModel {
        id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
        app_id: Set(app.id),
        expires_in: Set(NaiveDateTime::MAX),
        redirect_uri: Set(req.redirect_uri.clone()),
        token: Set(randstr::gen(Some(String::from("bda")))),
        resource_owner_id: NotSet,
        scopes: Set(req.scope.clone().unwrap_or(app.scopes)),
    };
    let grant = grant.insert(db.as_ref()).await.unwrap();

    let token = grant.token;

    let html_content = format!(
        r#"<!doctype html>
<html>
  <body>
    <form method="post">
        <label>
            Email:
            <input name="email" type="text" autocomplete="off">
        </label>
        <label>
            Password:
            <input name="password" type="password" autocomplete="off">
        </label>
        <input name="token" type="hidden" value="{token}">
        <input type="submit">
    </form>
  </body>
</html>"#
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[derive(Deserialize)]
pub struct AuthorizePostRequest {
    pub email: String,
    pub password: String,
    pub token: String,
}

pub async fn post(
    req: web::Form<AuthorizePostRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let user = match User::find()
        .filter(user::Column::Email.eq(req.email.clone()))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body("Username or password is incorrect."),
    };

    if !bcrypt::verify(&req.password, &user.encrypted_password).unwrap() {
        return HttpResponse::Unauthorized().body("Username or password is incorrect.");
    }

    let grant = match OauthAccessGrant::find()
        .filter(oauth_access_grant::Column::Token.eq(req.token.clone()))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(grant) => grant,
        None => return HttpResponse::BadRequest().finish(),
    };

    let redirect_uri = grant.redirect_uri.clone();

    let mut grant: oauth_access_grant::ActiveModel = grant.into();
    grant.resource_owner_id = Set(Some(user.id));
    let grant = grant.update(db.as_ref()).await.unwrap();

    let token = grant.token;

    if redirect_uri == "urn:ietf:wg:oauth:2.0:oob" {
        let html_content = format!(
            r#"<!doctype html>
<body>
Your code is <code>{token}</code>.
</body>"#
        );

        HttpResponse::Ok()
            .content_type("text/html")
            .body(html_content)
    } else {
        HttpResponse::Found()
            .insert_header((header::LOCATION, format!("{redirect_uri}?code={token}")))
            .finish()
    }
}
