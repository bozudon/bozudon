use crate::api_v1::utils::*;
use crate::entity;
use crate::model::prelude::*;
use crate::model::*;
use crate::Config;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::*;

pub async fn get(
    auth: Option<BearerAuth>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let status_ids = Favorite::find()
        .filter(Condition::all().add(favorite::Column::AccountId.eq(account_id)))
        .order_by_desc(favorite::Column::UpdatedAt)
        .limit(20)
        .all(db.as_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.status_id)
        .collect::<Vec<i64>>();

    HttpResponse::Ok().json(
        entity::retrieve_status_entities_from_db(
            db.as_ref(),
            config.as_ref(),
            &status_ids,
            Some(account_id),
        )
        .await,
    )
}
