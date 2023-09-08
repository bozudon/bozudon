use crate::api_v1::utils::*;
use crate::entity;
use crate::model::prelude::*;
use crate::model::*;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
pub struct Request {
    pub q: String,
}

#[derive(Serialize)]
pub struct Response {
    pub statuses: Vec<entity::Status>,
    pub accounts: Vec<entity::Account>,
    pub hashtags: Vec<serde_json::Value>,
}

async fn search_accounts(
    query: String,
    db: &DatabaseConnection,
    config: &crate::config::Config,
) -> Option<Vec<entity::Account>> {
    let query = query.split("@").filter(|x| *x != "").collect::<Vec<&str>>();
    if query.len() != 1 && query.len() != 2 {
        return None;
    }

    let only_local_search = match query.len() {
        1 => true,
        2 => query[1] == config.uri,
        _ => unreachable!(),
    };

    if only_local_search {
        let accounts = Account::find()
            .filter(account::Column::Username.contains(query[0]))
            .all(db)
            .await
            .unwrap()
            .into_iter()
            .map(|x| entity::account_model_to_entity(&x, 0, 0, 0)) // FIXME
            .collect::<Vec<entity::Account>>();
        return Some(accounts);
    }

    let username = query[0];
    let domain = query[1];
    let schema = if domain.starts_with("localhost") {
        "http"
    } else {
        "https"
    };

    // FIXME: Should check /.well-known/host-meta and /.well-known/webfinger
    let account = crate::activitypub::fetch_account(
        db,
        format!("{}://{}/users/{}", schema, domain, username),
        config,
        true,
    )
    .await;
    match account {
        None => None,
        Some(account) => Some(vec![entity::account_model_to_entity(&account, 0, 0, 0)]), // FIXME
    }
}

async fn search_statuses(
    query: String,
    db: &DatabaseConnection,
    config: &crate::config::Config,
    my_account_id: Option<i64>,
) -> Option<Vec<entity::Status>> {
    match url::Url::parse(query.as_str()) {
        Ok(_) => (),
        Err(_) => return None,
    };

    let status = crate::activitypub::fetch_status(db, query, config, true)
        .await
        .unwrap();
    Some(
        entity::retrieve_status_entities_from_db(db, config, &vec![status.id], my_account_id).await,
    )
}

pub async fn get(
    auth: Option<BearerAuth>,
    db: web::Data<DatabaseConnection>,
    req: web::Query<Request>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let _account = match Account::find_by_id(account_id).one(db.as_ref()).await {
        Ok(Some(account)) => account,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    return HttpResponse::Ok().json(Response {
        statuses: search_statuses(
            req.q.clone(),
            db.as_ref(),
            config.as_ref(),
            Some(account_id),
        )
        .await
        .unwrap_or(vec![]),
        accounts: search_accounts(req.q.clone(), db.as_ref(), config.as_ref())
            .await
            .unwrap_or(vec![]),
        hashtags: vec![],
    });
}
