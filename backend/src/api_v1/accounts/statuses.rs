use crate::api_v1::utils::*;
use crate::entity;
use crate::model::prelude::*;
use crate::model::*;
use crate::Config;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TargetAccountPath {
    pub id: String,
}

#[derive(Deserialize)]
pub struct AccountStatusesGetRequest {
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
    pub limit: Option<u64>,
    pub only_media: Option<bool>,
    pub exclude_replies: Option<bool>,
    pub exclude_reblogs: Option<bool>,
    pub pinned: Option<bool>,
    pub tagged: Option<String>,
}

pub async fn account_statuses_get(
    auth: Option<BearerAuth>,
    path: web::Path<TargetAccountPath>,
    req: web::Query<AccountStatusesGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut statuses_query = Status::find();

    let target_account_id: i64 = path.id.clone().parse().unwrap();
    statuses_query = statuses_query.filter(status::Column::AccountId.eq(target_account_id));

    let item_limit = req.limit.unwrap_or(20);
    if item_limit > 40 {
        // TODO: ここでいい感じのエラーを返す
    }

    if req.max_id.is_some() {
        match Status::find_by_id(req.max_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(max_item) => {
                statuses_query =
                    statuses_query.filter(status::Column::UpdatedAt.lt(max_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    if req.since_id.is_some() {
        match Status::find_by_id(req.since_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(since_item) => {
                statuses_query =
                    statuses_query.filter(status::Column::UpdatedAt.gt(since_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    if req.min_id.is_some() {
        match Status::find_by_id(req.min_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(min_item) => {
                statuses_query =
                    statuses_query.filter(status::Column::UpdatedAt.gt(min_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    // TODO: only_media

    if req.exclude_replies.is_some() {
        statuses_query = statuses_query.filter(status::Column::InReplyToId.is_null());
    }

    if req.exclude_reblogs.is_some() {
        statuses_query = statuses_query.filter(status::Column::ReblogOfId.is_null());
    }

    // TODO: pinned

    // TODO: tagged

    let status_ids = statuses_query
        .order_by_desc(status::Column::UpdatedAt)
        .limit(item_limit)
        .select_only()
        .column_as(status::Column::Id, "id")
        .into_tuple::<i64>()
        .all(db.as_ref())
        .await
        .unwrap();

    let statuses = entity::retrieve_status_entities_from_db(
        db.as_ref(),
        config.as_ref(),
        &status_ids,
        Some(account_id),
    )
    .await;
    HttpResponse::Ok().json(statuses)
}
