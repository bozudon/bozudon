use crate::entity;
use sea_orm::sea_query::Condition;
use sea_orm::{query::*, ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Serialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub url: String,
    pub display_name: String,
    pub created_at: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub statuses_count: i64,
}

#[derive(Serialize)]
pub struct Status {
    pub id: String,
    pub created_at: String,
    pub content: String,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub account: Account,
    pub reblog: Option<Box<Status>>,
    pub spoiler_text: String,
    pub media_attachments: Vec<MediaAttachment>,
    pub favourited: bool,
    pub favourites_count: i64,
    pub reblogged: bool,
    pub reblogs_count: i64,
}

#[derive(Serialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize)]
pub struct ApplicationWithoutClientInfo {
    pub name: String,
    pub website: Option<String>,
}

#[derive(Serialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: i64,
}

#[derive(Serialize)]
pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}

fn to_iso8601(date: &chrono::NaiveDateTime) -> String {
    date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub fn account_model_to_entity(
    account: &crate::model::account::Model,
    followers_count: i64,
    following_count: i64,
    statuses_count: i64,
) -> entity::Account {
    entity::Account {
        id: account.id.to_string(),
        username: account.username.clone(),
        acct: match account.domain.clone() {
            None => account.username.clone(),
            Some(domain) => format!("{}@{}", account.username, domain),
        },
        url: account.url.clone().unwrap_or(account.uri.clone()),
        display_name: account.display_name.clone(),
        created_at: to_iso8601(&account.created_at),
        followers_count,
        following_count,
        statuses_count,
    }
}

pub fn media_model_to_entity(
    media: crate::model::media::Model,
    config: &crate::Config,
) -> entity::MediaAttachment {
    match media.remote_url {
        Some(remote_url) => MediaAttachment {
            id: media.id.to_string(),
            media_type: "image".to_string(),
            url: remote_url.clone(),
            preview_url: remote_url,
            remote_url: None,
            text_url: None,
            meta: (),
            description: media.description,
            blurhash: media.blurhash,
        },
        None => {
            let media_type = match media.media_type.as_str() {
                "image/png" => String::from("image"),
                "image/jpeg" => String::from("image"),
                _ => todo!("Support more media types"),
            };
            MediaAttachment {
                id: media.id.to_string(),
                media_type,
                url: format!(
                    "{}/system/media_attachments/files/{}",
                    config.server_url, media.key
                ),
                preview_url: format!(
                    "{}/system/media_attachments/files/{}",
                    config.server_url, media.preview_key
                ),
                remote_url: None,
                text_url: None,
                meta: (),
                description: media.description,
                blurhash: media.blurhash,
            }
        }
    }
}

pub fn status_model_to_entity(
    status: &crate::model::status::Model,
    account: &crate::model::account::Model,
    in_reply_to_account_id: Option<i64>,
    favourited: bool,
    favourites_count: i64,
    reblogged: bool,
    reblogs_count: i64,
    reblog: Option<entity::Status>,
    attachments: &HashMap<i64, entity::MediaAttachment>,
) -> entity::Status {
    let media_attachments = status
        .media_ids
        .iter()
        .filter_map(|id| attachments.get(id))
        .map(|a| a.clone())
        .collect::<Vec<MediaAttachment>>();

    entity::Status {
        id: status.id.to_string(),
        created_at: to_iso8601(&status.created_at),
        content: status.text.clone(),
        in_reply_to_id: status.in_reply_to_id.map(|id| id.to_string()),
        in_reply_to_account_id: in_reply_to_account_id.map(|id| id.to_string()),
        account: account_model_to_entity(account, 0, 0, 0), // FIXME
        reblog: reblog.map(|reblog| Box::new(reblog)),
        spoiler_text: "".to_string(),
        media_attachments,
        favourited,
        favourites_count,
        reblogged,
        reblogs_count,
    }
}

pub async fn retrieve_media_entities_from_db<C>(
    db: &C,
    config: &crate::Config,
    ids: Vec<i64>,
) -> HashMap<i64, MediaAttachment>
where
    C: ConnectionTrait,
{
    use crate::model::prelude::*;
    use crate::model::*;

    Media::find()
        .filter(media::Column::Id.is_in(ids))
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|a| (a.id, media_model_to_entity(a, config)))
        .collect()
}

pub async fn retrieve_status_entities_from_db<C>(
    db: &C,
    config: &crate::Config,
    ids: &Vec<i64>,
    my_account_id: Option<i64>,
) -> Vec<Status>
where
    C: ConnectionTrait,
{
    use crate::model::prelude::*;
    use crate::model::*;

    let mut attachment_ids = HashSet::new();
    let mut m1 = HashMap::new();
    for (status, account) in Status::find()
        .filter(Condition::all().add(status::Column::Id.is_in(ids.clone())))
        .find_also_related(Account)
        .all(db)
        .await
        .unwrap()
    {
        status.media_ids.iter().for_each(|id| {
            attachment_ids.insert(id.clone());
        });
        m1.insert(status.id, (status, account.unwrap()));
    }

    let reblog_of_ids = m1
        .iter()
        .filter_map(|(_, (status, _account))| status.reblog_of_id)
        .collect::<Vec<i64>>();
    let mut m3 = HashMap::new();
    for (status, account) in Status::find()
        .filter(Condition::all().add(status::Column::Id.is_in(reblog_of_ids)))
        .find_also_related(Account)
        .all(db)
        .await
        .unwrap()
    {
        status.media_ids.iter().for_each(|id| {
            attachment_ids.insert(id.clone());
        });
        m3.insert(status.id, (status, account.unwrap()));
    }

    let attachments =
        retrieve_media_entities_from_db(db, config, attachment_ids.into_iter().collect()).await;

    let in_reply_to_ids = m1
        .iter()
        .chain(m3.iter())
        .filter_map(|(_, (status, _account))| status.in_reply_to_id)
        .collect::<Vec<i64>>();
    let mut m2 = HashMap::new();
    for status in Status::find()
        .filter(Condition::all().add(status::Column::Id.is_in(in_reply_to_ids)))
        .all(db)
        .await
        .unwrap()
        .iter()
    {
        m2.insert(status.id, status.account_id);
    }

    let ext_ids = m1
        .clone()
        .into_keys()
        .chain(m3.clone().into_keys())
        .collect::<Vec<i64>>();

    let favourites_count = Favorite::find()
        .filter(Condition::all().add(favorite::Column::StatusId.is_in(ext_ids.clone())))
        .select_only()
        .column_as(favorite::Column::StatusId, "id")
        .column_as(favorite::Column::Id.count(), "count")
        .group_by(favorite::Column::StatusId)
        .into_tuple::<(i64, i64)>()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|(id, count)| (id, count as i64))
        .collect::<HashMap<i64, i64>>();

    let favourited = match my_account_id {
        None => HashSet::new(),
        Some(my_account_id) => Favorite::find()
            .filter(
                Condition::all()
                    .add(favorite::Column::StatusId.is_in(ext_ids.clone()))
                    .add(favorite::Column::AccountId.eq(my_account_id)),
            )
            .select_only()
            .column_as(favorite::Column::StatusId, "id")
            .into_tuple()
            .all(db)
            .await
            .unwrap()
            .into_iter()
            .collect::<HashSet<i64>>(),
    };

    let reblogs_count = Status::find()
        .filter(Condition::all().add(status::Column::ReblogOfId.is_in(ext_ids.clone())))
        .select_only()
        .column_as(status::Column::ReblogOfId, "id")
        .column_as(status::Column::Id.count(), "count")
        .group_by(status::Column::ReblogOfId)
        .into_tuple::<(i64, i64)>()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|(id, count)| (id, count as i64))
        .collect::<HashMap<i64, i64>>();

    let reblogged = match my_account_id {
        None => HashSet::new(),
        Some(my_account_id) => Status::find()
            .filter(
                Condition::all()
                    .add(status::Column::ReblogOfId.is_in(ext_ids.clone()))
                    .add(status::Column::AccountId.eq(my_account_id)),
            )
            .select_only()
            .column_as(status::Column::ReblogOfId, "id")
            .into_tuple()
            .all(db)
            .await
            .unwrap()
            .into_iter()
            .collect::<HashSet<i64>>(),
    };

    ids.iter()
        .filter_map(|id| {
            m1.get(&id).map(|(s, a)| {
                let reblog = match s.reblog_of_id {
                    None => None,
                    Some(id) => {
                        let (s, a) = m3.get(&id).unwrap();
                        Some(status_model_to_entity(
                            s,
                            a,
                            s.in_reply_to_id.map(|id| *m2.get(&id).unwrap()),
                            favourited.contains(&id),
                            *favourites_count.get(&id).unwrap_or(&0),
                            reblogged.contains(&id),
                            *reblogs_count.get(&id).unwrap_or(&0),
                            None,
                            &attachments,
                        ))
                    }
                };
                status_model_to_entity(
                    s,
                    a,
                    s.in_reply_to_id.map(|id| *m2.get(&id).unwrap()),
                    favourited.contains(&id),
                    *favourites_count.get(&id).unwrap_or(&0),
                    reblogged.contains(&id),
                    *reblogs_count.get(&id).unwrap_or(&0),
                    reblog,
                    &attachments,
                )
            })
        })
        .collect()
}

#[derive(Serialize)]
pub struct AccountRelationship {
    pub id: String,
    pub following: bool,
    pub showing_reblogs: bool,
    pub notifying: bool,
    pub languages: Vec<String>,
    pub followed_by: bool,
    pub blocking: bool,
    pub blocked_by: bool,
    pub muting: bool,
    pub muting_notifications: bool,
    pub requested: bool,
    pub domain_blocking: bool,
    pub endorsed: bool,
    pub note: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct MediaAttachment {
    pub id: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub url: String,
    pub preview_url: String,
    pub remote_url: Option<String>,
    pub text_url: Option<String>,
    pub meta: (),
    pub description: String,
    pub blurhash: Option<String>,
}
