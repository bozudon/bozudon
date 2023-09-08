use crate::activitypub::{deliver_activity, naive_date_time_to_rfc3339, Activity};
use crate::api_v1::utils::*;
use crate::entity;
use crate::entity::{retrieve_status_entities_from_db, status_model_to_entity};
use crate::model::prelude::*;
use crate::model::*;
use crate::Config;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::sea_query::Condition;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    query::*, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait,
    FromQueryResult, IntoActiveModel, ModelTrait, QueryFilter, RelationTrait, Statement,
};
use serde::Deserialize;

async fn insert_new_status(
    status: status::ActiveModel,
    username: String,
    db: &DatabaseConnection,
    config: &crate::config::Config,
) -> status::Model {
    let status = status.insert(db).await.unwrap();
    let status_id = status.id;
    let mut status = status.into_active_model();
    status.uri = Set(format!(
        "https://{}/users/{}/statuses/{}",
        config.uri, username, status_id
    ));
    let status = status.update(db).await.unwrap();
    status
}

pub async fn get_followers(db: &DatabaseConnection, account_id: i64) -> Vec<account::Model> {
    Follow::find()
        .join(JoinType::LeftJoin, follow::Relation::Account2.def())
        .select_also(Account)
        .filter(
            Condition::all()
                .add(follow::Column::TargetAccountId.eq(account_id))
                .add(account::Column::Domain.is_not_null()),
        )
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|(_, account)| account.unwrap())
        .collect::<Vec<account::Model>>()
}

#[derive(Deserialize)]
pub struct StatusesPostRequest {
    pub status: String,
    pub media_ids: Vec<String>,
    pub spoiler_text: Option<String>,
    pub in_reply_to_id: Option<String>,
}

pub async fn post(
    auth: Option<BearerAuth>,
    req: web::Json<StatusesPostRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let account = match Account::find_by_id(account_id).one(db.as_ref()).await {
        Ok(Some(account)) => account,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    // Find the status to reply if any
    let in_reply_to = match req.in_reply_to_id.clone() {
        None => None,
        Some(id) => {
            let id = match id.parse::<i64>() {
                Ok(id) => id,
                Err(_) => return HttpResponse::BadRequest().finish(),
            };
            match Status::find_by_id(id).one(db.as_ref()).await.unwrap() {
                None => return HttpResponse::BadRequest().finish(),
                Some(status) => Some(status),
            }
        }
    };

    let media_ids = match req
        .media_ids
        .iter()
        .map(|e| match e.parse() {
            Ok(id) => Some(id),
            Err(_) => None,
        })
        .collect::<Option<Vec<i64>>>()
    {
        Some(ids) => ids,
        None => return HttpResponse::BadRequest().finish(),
    };

    let attachments =
        entity::retrieve_media_entities_from_db(db.as_ref(), config.as_ref(), media_ids.clone())
            .await;

    // Insert status to database
    let status = status::ActiveModel {
        id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
        deleted_at: NotSet,
        text: Set(req.status.clone()),
        in_reply_to_id: Set(in_reply_to.clone().map(|x| x.id)),
        reblog_of_id: NotSet,
        account_id: Set(account.id),
        uri: Set("".to_string()),
        url: NotSet,
        media_ids: Set(media_ids.clone()),
    };
    let status = insert_new_status(
        status,
        account.username.clone(),
        db.as_ref(),
        config.as_ref(),
    )
    .await;

    // Distribute the status to the followers
    let mut attachment = vec![];
    for id in media_ids.into_iter() {
        match Media::find_by_id(id).one(db.as_ref()).await {
            Ok(Some(media)) => attachment.push(Activity::Document {
                media_type: None,
                name: None,
                width: None,
                height: None,
                url: format!(
                    "{}/system/media_attachments/files/{}",
                    config.server_url, media.key,
                ),
            }),
            _ => (),
        }
    }
    let activity = Activity::Create {
        id: format!("{}/activity", status.uri.clone()),
        actor: account.uri.clone(),
        published: naive_date_time_to_rfc3339(status.created_at.clone()),
        to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
        cc: vec![account.followers_url.clone()],
        object: Box::new(Activity::Note {
            id: status.uri.clone(),
            summary: None,
            published: naive_date_time_to_rfc3339(status.created_at.clone()),
            url: status.uri.clone(),
            attributed_to: account.uri.clone(),
            in_reply_to: match in_reply_to {
                None => serde_json::Value::Null,
                Some(status) => serde_json::Value::String(status.uri.clone()),
            },
            to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
            cc: vec![account.followers_url.clone()],
            content: status.text.clone(),
            attachment: attachment,
        }),
    };
    let followers = get_followers(db.as_ref(), account_id).await;
    deliver_activity(activity, account.clone(), followers).await;

    // Return the result to the client
    let in_reply_to_account_id = match status.in_reply_to_id {
        None => None,
        Some(id) => match Status::find_by_id(id).one(db.as_ref()).await.unwrap() {
            None => None,
            Some(status) => Some(status.account_id),
        },
    };
    HttpResponse::Ok().json(status_model_to_entity(
        &status,
        &account,
        in_reply_to_account_id,
        false,
        0,
        false,
        0,
        None,
        &attachments,
    ))
}

#[derive(Deserialize)]
pub struct StatusesGetRequest {
    pub id: String,
}

pub async fn get(
    req: web::Path<StatusesGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    auth: Option<BearerAuth>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => Some(user.account_id),
        None => None,
    };

    let id = match req.id.clone().parse::<i64>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let xs =
        retrieve_status_entities_from_db(db.as_ref(), config.as_ref(), &vec![id], account_id).await;
    if xs.len() == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().json(&xs[0])
    }
}

pub async fn context(
    req: web::Path<StatusesGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    auth: Option<BearerAuth>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => Some(user.account_id),
        None => None,
    };

    let id = match req.id.clone().parse::<i64>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let ancestor_ids: Vec<i64> = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
WITH RECURSIVE t(id) AS (
    SELECT in_reply_to_id AS id FROM status WHERE id = $1
    UNION
    SELECT s.in_reply_to_id AS id FROM status s, t WHERE s.id = t.id
)
SELECT id FROM status WHERE id IN (SELECT id FROM t)
ORDER BY created_at
            "#,
        [id.into()],
    ))
    .all(db.as_ref())
    .await
    .unwrap()
    .into_iter()
    .map(|x| x.get("id").unwrap().as_i64().unwrap())
    .collect();

    let ancestors =
        retrieve_status_entities_from_db(db.as_ref(), config.as_ref(), &ancestor_ids, account_id)
            .await;

    let descendant_ids: Vec<i64> = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
WITH RECURSIVE t(id) AS (
    SELECT id FROM status WHERE in_reply_to_id = $1
    UNION
    SELECT s.id AS id FROM status s, t WHERE s.in_reply_to_id = t.id
)
SELECT id FROM status WHERE id IN (SELECT id FROM t)
ORDER BY created_at
            "#,
        [id.into()],
    ))
    .all(db.as_ref())
    .await
    .unwrap()
    .into_iter()
    .map(|x| x.get("id").unwrap().as_i64().unwrap())
    .collect();

    let descendants =
        retrieve_status_entities_from_db(db.as_ref(), config.as_ref(), &descendant_ids, account_id)
            .await;

    HttpResponse::Ok().json(entity::Context {
        ancestors,
        descendants,
    })
}

pub async fn favourite(
    auth: Option<BearerAuth>,
    req: web::Path<StatusesGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let account = match Account::find_by_id(account_id).one(db.as_ref()).await {
        Ok(Some(account)) => account,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    let id = match req.id.clone().parse::<i64>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // Check if the specified status exists
    let (status, status_account) = match Status::find_by_id(id)
        .find_also_related(Account)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some((status, Some(account))) => (status, account),
        _ => {
            return HttpResponse::NotFound().finish();
        }
    };

    // Favorite the status unless it's already favorited
    match Favorite::find()
        .filter(
            Condition::all()
                .add(favorite::Column::AccountId.eq(account_id))
                .add(favorite::Column::StatusId.eq(id)),
        )
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(_) => (),
        None => {
            let fav = favorite::ActiveModel {
                id: NotSet,
                created_at: NotSet,
                updated_at: NotSet,
                account_id: Set(account_id),
                status_id: Set(id),
            };
            let fav = fav.insert(db.as_ref()).await.unwrap();

            // Deliver Like activity if the author is remote
            if status_account.domain.is_some() {
                let activity = Activity::Like {
                    id: format!("{}#likes/{}", account.uri.clone(), fav.id),
                    actor: account.uri.clone(),
                    object: status.uri,
                };
                deliver_activity(activity, account, vec![status_account]).await;
            }
        }
    }

    // Return the status
    let xs =
        retrieve_status_entities_from_db(db.as_ref(), config.as_ref(), &vec![id], Some(account_id))
            .await;
    if xs.len() == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().json(&xs[0])
    }
}

pub async fn unfavourite(
    auth: Option<BearerAuth>,
    req: web::Path<StatusesGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let account = match Account::find_by_id(account_id).one(db.as_ref()).await {
        Ok(Some(account)) => account,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    let id = match req.id.clone().parse::<i64>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // Check if the specified status exists
    let (status, status_account) = match Status::find_by_id(id)
        .find_also_related(Account)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some((status, Some(account))) => (status, account),
        _ => {
            return HttpResponse::NotFound().finish();
        }
    };

    // Unfavorite the status if it's favorited
    match Favorite::find()
        .filter(
            Condition::all()
                .add(favorite::Column::AccountId.eq(account_id))
                .add(favorite::Column::StatusId.eq(id)),
        )
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(fav) => {
            let fav_id = fav.id;
            fav.delete(db.as_ref()).await.unwrap();

            // Deliver Undo/Like activity if the author is remote
            let activity = Activity::Undo {
                id: format!("{}#likes/{}/undo", account.uri.clone(), fav_id),
                actor: account.uri.clone(),
                object: Box::new(Activity::Like {
                    id: format!("{}#likes/{}", account.uri.clone(), fav_id),
                    actor: account.uri.clone(),
                    object: status.uri,
                }),
            };
            deliver_activity(activity, account, vec![status_account]).await;
        }
        None => (),
    }

    // Return the status
    let xs =
        retrieve_status_entities_from_db(db.as_ref(), config.as_ref(), &vec![id], Some(account_id))
            .await;
    if xs.len() == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().json(&xs[0])
    }
}

pub async fn unreblog(
    auth: Option<BearerAuth>,
    req: web::Path<StatusesGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let account = match Account::find_by_id(account_id).one(db.as_ref()).await {
        Ok(Some(account)) => account,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    let id = match req.id.clone().parse::<i64>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // Unreblog the status (i.e., delete the reblog) if reblogged
    match Status::find()
        .filter(
            Condition::all()
                .add(status::Column::AccountId.eq(account_id))
                .add(status::Column::ReblogOfId.eq(id)),
        )
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(reblog1) => {
            let reblog = reblog1.clone();
            reblog1.delete(db.as_ref()).await.unwrap();

            // Deliver Undo/Announce activity to remote followers and the remote author
            let mut dsts = get_followers(db.as_ref(), account_id).await;
            let (reblogged_status, reblogged_status_author) = match Status::find_by_id(id)
                .find_also_related(Account)
                .one(db.as_ref())
                .await
                .unwrap()
            {
                Some((status, Some(account))) => (status, account),
                _ => unreachable!(),
            };
            if reblogged_status_author.domain.is_some() {
                dsts.push(reblogged_status_author.clone());
            }
            let activity = Activity::Undo {
                id: format!("{}/undo", reblog.uri.clone()),
                actor: account.uri.clone(),
                object: Box::new(Activity::Announce {
                    id: reblog.uri,
                    actor: account.uri.clone(),
                    published: naive_date_time_to_rfc3339(reblog.created_at),
                    to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
                    cc: vec![account.followers_url.clone(), reblogged_status_author.uri],
                    object: reblogged_status.uri,
                }),
            };
            deliver_activity(activity, account, dsts).await;
        }
        None => (),
    };

    // Return the status
    let xs =
        retrieve_status_entities_from_db(db.as_ref(), config.as_ref(), &vec![id], Some(account_id))
            .await;
    if xs.len() == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().json(&xs[0])
    }
}

pub async fn reblog(
    auth: Option<BearerAuth>,
    req: web::Path<StatusesGetRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let account = match Account::find_by_id(account_id).one(db.as_ref()).await {
        Ok(Some(account)) => account,
        _ => return HttpResponse::Unauthorized().finish(),
    };

    let id = match req.id.clone().parse::<i64>() {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // Check if the specified status exists and it's not a reblog
    let (status, status_account) = match Status::find_by_id(id)
        .find_also_related(Account)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some((status, Some(status_account))) => {
            if status.reblog_of_id.is_some() {
                return HttpResponse::BadRequest().finish();
            }
            (status, status_account)
        }
        _ => {
            return HttpResponse::NotFound().finish();
        }
    };

    // Reblog the status unless it's already reblogged
    match Status::find()
        .filter(
            Condition::all()
                .add(status::Column::AccountId.eq(account_id))
                .add(status::Column::ReblogOfId.eq(id)),
        )
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(_) => (),
        None => {
            let reblog = status::ActiveModel {
                id: NotSet,
                created_at: NotSet,
                updated_at: NotSet,
                deleted_at: NotSet,
                text: Set("".to_string()),
                in_reply_to_id: NotSet,
                reblog_of_id: Set(Some(id)),
                account_id: Set(account_id),
                uri: Set("".to_string()),
                url: NotSet,
                media_ids: Set(vec![]),
            };
            let reblog = insert_new_status(
                reblog,
                account.username.clone(),
                db.as_ref(),
                config.as_ref(),
            )
            .await;

            // Send Announce activity to followers and the author, if the author is remote
            let mut dsts = get_followers(db.as_ref(), account_id).await;
            let reblogged_status_author = Status::find_by_id(id)
                .find_also_related(Account)
                .one(db.as_ref())
                .await
                .unwrap()
                .unwrap()
                .1
                .unwrap();
            if reblogged_status_author.domain.is_some() {
                dsts.push(reblogged_status_author);
            }
            let activity = Activity::Announce {
                id: reblog.uri,
                actor: account.uri.clone(),
                published: naive_date_time_to_rfc3339(reblog.created_at),
                to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
                cc: vec![account.followers_url.clone(), status_account.uri],
                object: status.uri,
            };
            deliver_activity(activity, account, dsts).await;
        }
    };

    // Return the status
    let xs =
        retrieve_status_entities_from_db(db.as_ref(), config.as_ref(), &vec![id], Some(account_id))
            .await;
    if xs.len() == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().json(&xs[0])
    }
}
