use crate::api_v1::utils::*;
use crate::entity;
use crate::model::prelude::*;
use crate::model::*;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TargetAccountPath {
    pub id: String,
}

#[derive(Deserialize)]
pub struct AccountFollowPostRequest {
    pub reblogs: Option<bool>,
    pub notify: Option<bool>,
    pub languages: Option<Vec<String>>,
}

pub async fn account_follow_post(
    auth: Option<BearerAuth>,
    path: web::Path<TargetAccountPath>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let target_account_id: i64 = path.id.clone().parse().unwrap();

    let account = match Account::find_by_id(account_id)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return HttpResponse::NotFound().finish(),
    };
    let target_account = match Account::find_by_id(target_account_id)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return HttpResponse::NotFound().finish(),
    };

    // Use transaction to rollback if the delivery of the Follow activity fails
    let result = db
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let follow = follow::ActiveModel {
                    id: NotSet,
                    created_at: NotSet,
                    updated_at: NotSet,
                    account_id: Set(account_id),
                    target_account_id: Set(target_account_id),
                };
                let follow = match follow.insert(txn).await {
                    Ok(follow) => Some(follow),
                    Err(_) => None,
                };

                // Deliver Follow activity if the target account is remote
                match (follow, target_account.domain) {
                    (Some(follow), Some(_)) => {
                        let activity = crate::activitypub::Activity::Follow {
                            id: format!("https://{}/follows/{}", config.uri, follow.id),
                            actor: account.uri.clone(),
                            object: target_account.uri,
                        };
                        match crate::activitypub::post_activity(
                            activity,
                            target_account.inbox_url,
                            Some(account),
                        )
                        .await
                        {
                            Ok(_) => Ok(()),
                            Err(_) => Err(DbErr::Custom("rollback".to_string())),
                        }
                    }
                    _ => Ok(()),
                }
            })
        })
        .await;
    if result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let followed_by = match Follow::find()
        .filter(follow::Column::AccountId.eq(target_account_id))
        .filter(follow::Column::TargetAccountId.eq(account_id))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(_) => true,
        None => false,
    };

    HttpResponse::Ok().json(entity::AccountRelationship {
        id: target_account_id.to_string(),
        following: true,
        showing_reblogs: false,
        notifying: false,
        languages: vec![],
        followed_by: followed_by,
        blocking: false,
        blocked_by: false,
        muting: false,
        muting_notifications: false,
        requested: false,
        domain_blocking: false,
        endorsed: false,
        note: "".to_string(),
    })
}

pub async fn account_unfollow_post(
    auth: Option<BearerAuth>,
    path: web::Path<TargetAccountPath>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };
    let target_account_id: i64 = path.id.clone().parse().unwrap();

    let account = match Account::find_by_id(account_id)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return HttpResponse::NotFound().finish(),
    };
    let target_account = match Account::find_by_id(target_account_id)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return HttpResponse::NotFound().finish(),
    };

    match Follow::find()
        .filter(follow::Column::AccountId.eq(account_id))
        .filter(follow::Column::TargetAccountId.eq(target_account_id))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(follow) => {
            let follow_id = follow.id;
            follow.delete(db.as_ref()).await.unwrap();

            // Deliver Undo/Follow activity if the target account is remote
            match target_account.domain {
                Some(_) => {
                    let activity = crate::activitypub::Activity::Undo {
                        id: format!("https://{}/follows/{}/undo", config.uri, follow_id),
                        actor: account.uri.clone(),
                        object: Box::new(crate::activitypub::Activity::Follow {
                            id: format!("https://{}/follows/{}", config.uri, follow_id),
                            actor: account.uri.clone(),
                            object: target_account.uri,
                        }),
                    };
                    match crate::activitypub::post_activity(
                        activity,
                        target_account.inbox_url,
                        Some(account),
                    )
                    .await
                    {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                None => {}
            }
        }
        None => {}
    };

    let followed_by = match Follow::find()
        .filter(follow::Column::AccountId.eq(target_account_id))
        .filter(follow::Column::TargetAccountId.eq(account_id))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(_) => true,
        None => false,
    };

    HttpResponse::Ok().json(entity::AccountRelationship {
        id: target_account_id.to_string(),
        following: false,
        showing_reblogs: false,
        notifying: false,
        languages: vec![],
        followed_by: followed_by,
        blocking: false,
        blocked_by: false,
        muting: false,
        muting_notifications: false,
        requested: false,
        domain_blocking: false,
        endorsed: false,
        note: "".to_string(),
    })
}

#[derive(Deserialize)]
pub struct AccountFollowersGetRequest {
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
    pub limit: Option<u64>,
}

pub async fn account_followers_get(
    path: web::Path<TargetAccountPath>,
    req: web::Query<AccountFollowersGetRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let account_id: i64 = path.id.clone().parse().unwrap();

    let item_limit = req.limit.unwrap_or(40);

    if item_limit > 40 {
        // TODO: ここでいい感じのエラーを返す
    }

    let mut followers_query = Follow::find().filter(follow::Column::TargetAccountId.eq(account_id));

    if req.max_id.is_some() {
        match Follow::find_by_id(req.max_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(max_item) => {
                followers_query =
                    followers_query.filter(follow::Column::UpdatedAt.lt(max_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    if req.since_id.is_some() {
        match Follow::find_by_id(req.since_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(since_item) => {
                followers_query =
                    followers_query.filter(follow::Column::UpdatedAt.gt(since_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    if req.min_id.is_some() {
        match Follow::find_by_id(req.min_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(min_item) => {
                followers_query =
                    followers_query.filter(follow::Column::UpdatedAt.gt(min_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    let followers = followers_query
        .join(JoinType::LeftJoin, follow::Relation::Account2.def())
        .select_also(Account)
        .order_by_desc(follow::Column::UpdatedAt)
        .limit(item_limit)
        .all(db.as_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|(_, account)| entity::account_model_to_entity(&account.unwrap(), 0, 0, 0)) // FIXME
        .collect::<Vec<entity::Account>>();

    HttpResponse::Ok().json(followers)
}

#[derive(Deserialize)]
pub struct AccountFollowingGetRequest {
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
    pub limit: Option<u64>,
}

pub async fn account_following_get(
    path: web::Path<TargetAccountPath>,
    req: web::Query<AccountFollowingGetRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let account_id: i64 = path.id.clone().parse().unwrap();

    let item_limit = req.limit.unwrap_or(40);

    if item_limit > 40 {
        // TODO: ここでいい感じのエラーを返す
    }

    let mut following_query = Follow::find().filter(follow::Column::AccountId.eq(account_id));

    if req.max_id.is_some() {
        match Follow::find_by_id(req.max_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(max_item) => {
                following_query =
                    following_query.filter(follow::Column::UpdatedAt.lt(max_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    if req.since_id.is_some() {
        match Follow::find_by_id(req.since_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(since_item) => {
                following_query =
                    following_query.filter(follow::Column::UpdatedAt.gt(since_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    if req.min_id.is_some() {
        match Follow::find_by_id(req.min_id.as_ref().unwrap().parse::<i64>().unwrap())
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(min_item) => {
                following_query =
                    following_query.filter(follow::Column::UpdatedAt.gt(min_item.updated_at))
            }
            None => (), // TODO: ここでいい感じのエラーを返す
        };
    }

    let following = following_query
        .join(JoinType::LeftJoin, follow::Relation::Account1.def())
        .select_also(Account)
        .order_by_desc(follow::Column::UpdatedAt)
        .limit(item_limit)
        .all(db.as_ref())
        .await
        .unwrap()
        .into_iter()
        .map(|(_, account)| entity::account_model_to_entity(&account.unwrap(), 0, 0, 0)) // FIXME
        .collect::<Vec<entity::Account>>();

    HttpResponse::Ok().json(following)
}
