use crate::model::prelude::*;
use crate::randstr;
use crate::{entity, model::*};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{entity::*, ActiveModelTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
    client_id: String,
    client_secret: String,
}

pub async fn post(
    req: web::Json<LoginRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let app = match App::find()
        .filter(app::Column::ClientId.eq(req.client_id.clone()))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(app) => app,
        None => return HttpResponse::BadRequest().finish(),
    };
    if app.client_secret != req.client_secret {
        return HttpResponse::BadRequest().finish();
    }

    let user = match User::find()
        .filter(user::Column::Email.eq(req.email.clone()))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(user) => user,
        None => {
            return HttpResponse::Unauthorized().json(
                [("error", "Username or password is incorrect")]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            )
        }
    };

    if !bcrypt::verify(&req.password, &user.encrypted_password).unwrap() {
        return HttpResponse::Unauthorized().json(
            [("error", "Username or password is incorrect")]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        );
    }

    let access_token = oauth_access_token::ActiveModel {
        id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
        token: Set(String::from(randstr::gen(Some(String::from("bdt"))))),
        app_id: Set(app.id),
        resource_owner_id: Set(user.id),
        scopes: Set(String::from("read write follow push")),
    };

    let access_token = access_token.insert(db.as_ref()).await.unwrap();

    HttpResponse::Ok().json(entity::Token {
        access_token: access_token.token,
        token_type: String::from("Bearer"),
        scope: String::from(access_token.scopes),
        created_at: access_token.created_at.timestamp(),
    })
}
