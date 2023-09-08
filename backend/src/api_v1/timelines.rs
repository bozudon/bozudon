use crate::api_v1::utils::*;
use crate::model::prelude::*;
use crate::model::*;
use crate::Config;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TimelinesHomeGetRequest {
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
    pub limit: Option<u64>,
}

pub async fn home(
    auth: Option<BearerAuth>,
    req: web::Query<TimelinesHomeGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut statuses_query = Status::find();

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

    let followers = Follow::find()
        .filter(follow::Column::AccountId.eq(account_id))
        .all(db.as_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|follow| follow.target_account_id)
        .collect::<Vec<i64>>();

    let status_ids = statuses_query
        .filter(
            Condition::any()
                .add(status::Column::AccountId.eq(account_id))
                .add(status::Column::AccountId.is_in(followers)),
        )
        .limit(item_limit)
        .order_by_desc(status::Column::UpdatedAt)
        .select_only()
        .column_as(status::Column::Id, "id")
        .into_tuple::<i64>()
        .all(db.as_ref())
        .await
        .unwrap();

    let statuses = crate::entity::retrieve_status_entities_from_db(
        db.as_ref(),
        config.as_ref(),
        &status_ids,
        Some(account_id),
    )
    .await;
    HttpResponse::Ok().json(statuses)
}
