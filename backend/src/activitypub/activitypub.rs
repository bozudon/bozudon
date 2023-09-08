use crate::activitypub::activity;
use crate::model::prelude::*;
use crate::model::*;
use actix_web::{http::header, web, HttpResponse, Responder};
use async_recursion::async_recursion;
use base64::Engine;
use lazy_static::__Deref;
use log::{debug, error, info};
use openssl::{pkey::Private, rsa::Rsa};
use reqwest::Client;
use sea_orm::{
    sea_query::Condition, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, IntoActiveModel, ModelTrait, NotSet, QueryFilter, Set,
};
use serde::Deserialize;
use sha2::Digest;
use std::str;

fn build_signinig_string(
    signed_headers: Vec<String>,
    headers: Vec<(String, String)>,
    meth: String,
    path: String,
) -> String {
    let pseudo_headers = headers
        .into_iter()
        .map(|x| (x.0.to_lowercase(), x.1))
        .collect::<Vec<(String, String)>>();
    let meth = meth.to_lowercase();
    signed_headers
        .into_iter()
        .map(|x| match x.as_str() {
            "(request-target)" => format!("(request-target): {} {}", meth, path),
            "(created)" | "(expires)" => panic!("unimplemented pseudo header"),
            header => {
                let values = pseudo_headers
                    .clone()
                    .into_iter()
                    .filter_map(|x| if x.0 == header { Some(x.1) } else { None })
                    .collect::<Vec<String>>();
                if values.len() == 0 {
                    panic!("Specified signed header not found")
                }
                format!("{}: {}", header, values.join(","))
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn may_push_digest_header(
    prefix: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
) -> Vec<(String, String)> {
    match body {
        None => headers,
        Some(body) => {
            let digest = format!(
                "{}={}",
                prefix,
                base64::engine::general_purpose::STANDARD
                    .encode(sha2::Sha256::digest(body.as_bytes()))
            );
            let mut headers = headers;
            headers.push(("digest".to_string(), digest));
            headers
        }
    }
}

fn sign(
    priv_key: Rsa<Private>,
    key_id: String,
    signed_headers: Vec<String>,
    headers: Vec<(String, String)>,
    meth: String,
    path: String,
    body: Option<String>,
) -> Vec<(String, String)> {
    let algorithm = "rsa-sha256";
    let mut headers = may_push_digest_header("SHA-256".to_string(), headers, body);
    let signing_string = build_signinig_string(signed_headers.clone(), headers.clone(), meth, path);
    debug!("signing_string: {}", signing_string);
    let signature = {
        let pkey = openssl::pkey::PKey::from_rsa(priv_key).unwrap();
        let mut signer =
            openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(), &pkey).unwrap();
        signer
            .set_rsa_padding(openssl::rsa::Padding::PKCS1)
            .unwrap();
        let tag = signer
            .sign_oneshot_to_vec(signing_string.as_bytes())
            .unwrap();
        base64::engine::general_purpose::STANDARD.encode(&tag)
    };
    headers.push((
        "signature".to_string(),
        format!(
            "keyId=\"{}\",algorithm=\"{}\",headers=\"{}\",signature=\"{}\"",
            key_id,
            algorithm,
            signed_headers.join(" "),
            signature
        ),
    ));
    headers
}

pub async fn deliver_activity(
    activity: activity::Activity,
    src: account::Model,
    dsts: Vec<account::Model>,
) {
    for dst in dsts {
        match post_activity(activity.clone(), dst.inbox_url, Some(src.clone())).await {
            Ok(()) => (),
            Err(e) => error!("deliver_activity: {:?}", e),
        }
    }
}

pub async fn post_activity(
    activity: activity::Activity,
    url: String,
    src: Option<account::Model>,
) -> Result<(), &'static str> {
    let parsed_url = url::Url::parse(url.as_str()).unwrap();
    let path = parsed_url.path().to_string();
    let method = "post".to_string();

    let body = serde_json::to_string(&activity::WithContext::new(activity)).unwrap();
    let signed_headers = vec![
        "(request-target)".to_string(),
        "host".to_string(),
        "date".to_string(),
        "digest".to_string(),
        "content-type".to_string(),
    ];
    let headers = {
        let host = match parsed_url.host() {
            Some(url::Host::Domain(s)) => s.to_string(),
            _ => return Err("Invalid host of url"),
        };
        let host = match parsed_url.port() {
            None => host,
            Some(port) => format!("{}:{}", host, port),
        };
        let date = chrono::Utc::now().format("%a, %d %b %Y %T GMT").to_string();
        vec![
            (
                "content-type".to_string(),
                "application/activity+json".to_string(),
            ),
            ("host".to_string(), host),
            ("date".to_string(), date),
        ]
    };
    let headers = match src {
        Some(src) => {
            let key_id = format!("{}#main-key", src.uri);
            let priv_key =
                openssl::rsa::Rsa::private_key_from_pem(src.private_key.unwrap().as_bytes())
                    .unwrap();
            sign(
                priv_key,
                key_id,
                signed_headers,
                headers,
                method,
                path,
                Some(body.clone()),
            )
        }
        _ => headers,
    };
    let mut headers_map = reqwest::header::HeaderMap::new();
    for (k, v) in headers.iter() {
        debug!("Header {}: {}", k, v);
        headers_map.insert(
            reqwest::header::HeaderName::from_lowercase(k.to_lowercase().as_bytes()).unwrap(),
            reqwest::header::HeaderValue::from_str(v).unwrap(),
        );
    }
    debug!("url: {}", url);
    debug!("body: {:?}", body);
    debug!("headers: {:?}", headers_map);

    let res = Client::new()
        .post(url)
        .body(body)
        .headers(headers_map)
        .send()
        .await;

    match res {
        Err(_) => Err("connection failure"),
        Ok(resp) => {
            if resp.status().is_success() {
                Ok(())
            } else {
                error!(
                    "post_activity: {} {}",
                    resp.status().to_string(),
                    resp.text().await.unwrap_or("".to_string())
                );
                Err("status is not success")
            }
        }
    }
}

pub async fn fetch_activity(uri: String) -> Option<String> {
    info!("fetch_activity: {}", uri);
    match Client::new()
        .get(uri)
        .header(header::ACCEPT, "application/activity+json")
        .send()
        .await
    {
        Err(_) => None,
        Ok(resp) => Some(resp.text().await.unwrap()),
    }
}

pub async fn fetch_account<C>(
    db: &C,
    uri: String,
    config: &crate::config::Config,
    resolve: bool, // true iff we should fetch the account from the remote server
) -> Option<account::Model>
where
    C: ConnectionTrait,
{
    info!("fetch_account: {}", uri);

    if let Some(a) = Account::find()
        .filter(Condition::all().add(account::Column::Uri.eq(&uri)))
        .one(db)
        .await
        .unwrap()
    {
        return Some(a);
    }
    if !resolve {
        return None;
    }

    let parsed_uri = match url::Url::parse(&uri) {
        Ok(u) => u,
        Err(_) => return None,
    };
    let domain = match parsed_uri.host() {
        Some(url::Host::Domain(s)) => {
            if s == config.uri {
                return None;
            }
            match parsed_uri.port() {
                None => s.to_string(),
                Some(p) => format!("{}:{}", s, p),
            }
        }
        _ => return None,
    };

    let body = match fetch_activity(uri).await {
        None => return None,
        Some(s) => s,
    };
    debug!("body: {}", body);
    let activity = match serde_json::from_str::<activity::WithContext<activity::Activity>>(&body) {
        Ok(a) => a.data,
        Err(_) => return None,
    };
    match activity {
        activity::Activity::Person {
            id,
            followers,
            inbox,
            outbox,
            endpoints: activity::PersonEndpoints { shared_inbox },
            preferred_username,
            name,
            summary,
            url,
            public_key,
            ..
        } => {
            let account = account::ActiveModel {
                id: NotSet,
                created_at: NotSet,
                updated_at: NotSet,
                username: Set(preferred_username),
                domain: Set(Some(domain.to_string())),
                display_name: Set(name),
                note: Set(summary),
                uri: Set(id),
                url: Set(Some(url)),
                inbox_url: Set(inbox),
                outbox_url: Set(outbox),
                shared_inbox_url: Set(shared_inbox),
                followers_url: Set(followers),
                private_key: NotSet,
                public_key: Set(public_key.public_key_pem),
            };
            Some(account.insert(db).await.unwrap())
        }
        _ => None,
    }
}

#[async_recursion]
pub async fn fetch_status_from_activity(
    db: &DatabaseConnection,
    activity: activity::Activity,
    config: &crate::config::Config,
) -> Option<status::Model> {
    match activity {
        activity::Activity::Note {
            id,
            published,
            url,
            attributed_to,
            content,
            in_reply_to,
            attachment,
            ..
        } => {
            if let Some(s) = Status::find()
                .filter(Condition::all().add(status::Column::Uri.eq(&id)))
                .one(db)
                .await
                .unwrap()
            {
                return Some(s);
            }

            let account_id = match fetch_account(db, attributed_to.clone(), config, true).await {
                None => return None,
                Some(a) => a.id,
            };

            let published = chrono::DateTime::parse_from_rfc3339(published.as_ref())
                .unwrap()
                .naive_utc();

            let in_reply_to = match in_reply_to {
                serde_json::Value::String(s) => {
                    Some(fetch_status(db, s.clone(), config, true).await.unwrap().id)
                }
                _ => None,
            };

            let mut media_ids = vec![];
            for a in attachment {
                let remote_url = match a {
                    activity::Activity::Document { url, .. } => url,
                    _ => continue,
                };
                let key = format!("{:02X}", sha2::Sha256::digest(remote_url.as_bytes()));
                let media = media::ActiveModel {
                    id: NotSet,
                    created_at: NotSet,
                    updated_at: NotSet,
                    key: Set(key.clone()),
                    preview_key: Set(key),
                    media_type: Set("image/png".to_string()),
                    description: Set("".to_string()),
                    blurhash: NotSet,
                    remote_url: Set(Some(remote_url)),
                    resource_owner_id: NotSet,
                };
                match media.insert(db).await {
                    Err(_) => (),
                    Ok(m) => media_ids.push(m.id),
                }
            }

            let status = status::ActiveModel {
                id: NotSet,
                created_at: Set(published),
                updated_at: Set(published),
                deleted_at: NotSet,
                text: Set(content.clone()),
                in_reply_to_id: Set(in_reply_to),
                reblog_of_id: NotSet,
                account_id: Set(account_id),
                uri: Set(id.clone()),
                url: Set(Some(url.clone())),
                media_ids: Set(media_ids),
            }
            .insert(db)
            .await
            .unwrap();

            Some(status)
        }
        _ => None,
    }
}

#[async_recursion]
pub async fn fetch_status(
    db: &DatabaseConnection,
    uri: String,
    config: &crate::config::Config,
    resolve: bool,
) -> Option<status::Model> {
    info!("fetch_status: {}", uri);

    if let Some(s) = Status::find()
        .filter(Condition::all().add(status::Column::Uri.eq(&uri)))
        .one(db)
        .await
        .unwrap()
    {
        return Some(s);
    }
    if !resolve {
        return None;
    }

    let body = match fetch_activity(uri).await {
        None => return None,
        Some(s) => s,
    };
    debug!("body: {}", body);
    let activity = match serde_json::from_str::<activity::WithContext<activity::Activity>>(&body) {
        Ok(a) => a.data,
        Err(_) => return None,
    };
    fetch_status_from_activity(db, activity, config).await
}

pub async fn handle_activity_create_note(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    note: activity::Activity,
) {
    info!("inbox activity Create/Note");
    fetch_status_from_activity(db, note, config).await;
}

pub async fn handle_activity_follow(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    id: String,
    actor: String,
    object: String,
) {
    info!("inbox activity Follow");

    let account_id = match fetch_account(db, actor.clone(), config, true).await {
        None => return,
        Some(a) => a.id,
    };
    let target_account_id = match fetch_account(db, object.clone(), config, false).await {
        None => return,
        Some(a) => a.id,
    };

    match Follow::find()
        .filter(
            Condition::all()
                .add(follow::Column::AccountId.eq(account_id))
                .add(follow::Column::TargetAccountId.eq(target_account_id)),
        )
        .one(db)
        .await
        .unwrap()
    {
        Some(_) => (),
        None => {
            let follow = follow::ActiveModel {
                id: NotSet,
                created_at: NotSet,
                updated_at: NotSet,
                account_id: Set(account_id),
                target_account_id: Set(target_account_id),
            }
            .insert(db)
            .await
            .unwrap();

            // send Accept
            let followee = Account::find_by_id(target_account_id)
                .one(db)
                .await
                .unwrap()
                .unwrap();
            let follower = Account::find_by_id(account_id)
                .one(db)
                .await
                .unwrap()
                .unwrap();
            let activity = activity::Activity::Accept {
                id: format!("{}#accepts/follows/{}", followee.uri, follow.id),
                actor: followee.uri.clone(),
                object: Box::new(activity::Activity::Follow { id, actor, object }),
            };
            debug!("activity: {:?}", activity);
            match post_activity(activity, follower.inbox_url, Some(followee)).await {
                Ok(()) => (),
                Err(e) => error!("post_activity: {:?}", e),
            }
        }
    };
}

pub async fn handle_activity_announce(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    id: String,
    actor: String,
    published: String,
    _to: Vec<String>,
    _cc: Vec<String>,
    object: String,
) {
    match Status::find()
        .filter(Condition::all().add(status::Column::Uri.eq(id.clone())))
        .one(db)
        .await
        .unwrap()
    {
        Some(_) => return,
        None => (),
    };

    let account = match fetch_account(db, actor.clone(), config, true).await {
        None => return,
        Some(a) => a,
    };
    let status = match fetch_status(db, object.clone(), config, true).await {
        None => return,
        Some(s) => s,
    };
    let published = chrono::DateTime::parse_from_rfc3339(published.as_ref())
        .unwrap()
        .naive_utc();
    let reblog = status::ActiveModel {
        id: NotSet,
        created_at: Set(published),
        updated_at: Set(published),
        deleted_at: NotSet,
        text: Set("".to_string()),
        in_reply_to_id: NotSet,
        reblog_of_id: Set(Some(status.id)),
        account_id: Set(account.id),
        uri: Set(id.clone()),
        url: Set(None),
        media_ids: Set(vec![]),
    };
    reblog.insert(db).await.unwrap();
}

pub async fn handle_activity_like(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    _id: String,
    actor: String,
    object: String,
) {
    let account = match fetch_account(db, actor.clone(), config, true).await {
        None => return,
        Some(a) => a,
    };
    let status = match fetch_status(db, object.clone(), config, true).await {
        None => return,
        Some(s) => s,
    };
    match Favorite::find()
        .filter(
            Condition::all()
                .add(favorite::Column::AccountId.eq(account.id))
                .add(favorite::Column::StatusId.eq(status.id)),
        )
        .one(db)
        .await
        .unwrap()
    {
        Some(_) => return,
        None => (),
    };
    let fav = favorite::ActiveModel {
        id: NotSet,
        created_at: NotSet,
        updated_at: NotSet,
        account_id: Set(account.id),
        status_id: Set(status.id),
    };
    fav.insert(db).await.unwrap();
}

pub async fn handle_activity_undo_follow(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    _id: String,
    actor: String,
    object: String,
) {
    let account_id = match fetch_account(db, actor.clone(), config, true).await {
        None => return,
        Some(a) => a.id,
    };
    let target_account_id = match fetch_account(db, object.clone(), config, false).await {
        None => return,
        Some(a) => a.id,
    };

    match Follow::find()
        .filter(
            Condition::all()
                .add(follow::Column::AccountId.eq(account_id))
                .add(follow::Column::TargetAccountId.eq(target_account_id)),
        )
        .one(db)
        .await
        .unwrap()
    {
        None => (),
        Some(f) => {
            f.delete(db).await.unwrap();
        }
    }
}

pub async fn handle_activity_undo_like(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    _id: String,
    actor: String,
    object: String,
) {
    let account = match fetch_account(db, actor.clone(), config, true).await {
        None => return,
        Some(a) => a,
    };
    let status = match fetch_status(db, object.clone(), config, false).await {
        None => return,
        Some(s) => s,
    };

    match Favorite::find()
        .filter(
            Condition::all()
                .add(favorite::Column::AccountId.eq(account.id))
                .add(favorite::Column::StatusId.eq(status.id)),
        )
        .one(db)
        .await
        .unwrap()
    {
        None => (),
        Some(f) => {
            f.delete(db).await.unwrap();
        }
    }
}

pub async fn handle_activity_undo_announce(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    actor: String,
    object: String,
) {
    let account = match fetch_account(db, actor.clone(), config, true).await {
        None => return,
        Some(a) => a,
    };
    let status = match fetch_status(db, object.clone(), config, false).await {
        None => return,
        Some(s) => s,
    };

    match Status::find()
        .filter(
            Condition::all()
                .add(status::Column::AccountId.eq(account.id))
                .add(status::Column::ReblogOfId.eq(status.id)),
        )
        .one(db)
        .await
        .unwrap()
    {
        Some(reblog) => {
            reblog.delete(db).await.unwrap();
        }
        None => (),
    };
}

pub async fn handle_activity_update_person(
    db: &DatabaseConnection,
    config: &crate::config::Config,
    id: String,
    _following: String,
    _followers: String,
    _inbox: String,
    _outbox: String,
    _endpoints: activity::PersonEndpoints,
    _preferred_username: String,
    name: String,
    summary: String,
    _url: String,
    _tag: Vec<String>,
    _public_key: activity::PersonPublicKey,
    _icon: Option<Box<activity::Activity>>,
    _image: Option<Box<activity::Activity>>,
) {
    let mut account = match fetch_account(db, id.clone(), config, true).await {
        None => return,
        Some(a) => a,
    }
    .into_active_model();
    account.display_name = Set(name);
    account.note = Set(summary);
    account.update(db).await.unwrap();
}

pub async fn inbox(
    raw: web::Bytes,
    db: web::Data<DatabaseConnection>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    info!("inbox: {:?}", raw);

    match serde_json::from_slice::<activity::WithContext<activity::Activity>>(&raw)
        .unwrap()
        .data
    {
        activity::Activity::Follow { id, actor, object } => {
            handle_activity_follow(db.as_ref(), config.as_ref(), id, actor, object).await
        }
        activity::Activity::Create { object, .. } => match object.deref() {
            activity::Activity::Note { .. } => {
                handle_activity_create_note(db.as_ref(), config.as_ref(), object.deref().clone())
                    .await
            }
            _ => (),
        },
        activity::Activity::Announce {
            id,
            actor,
            published,
            to,
            cc,
            object,
        } => {
            handle_activity_announce(
                db.as_ref(),
                config.as_ref(),
                id,
                actor,
                published,
                to,
                cc,
                object.deref().to_string(),
            )
            .await
        }
        activity::Activity::Like { id, actor, object } => {
            handle_activity_like(db.as_ref(), config.as_ref(), id, actor, object).await
        }
        activity::Activity::Undo { object, .. } => match *object {
            activity::Activity::Follow { id, actor, object } => {
                handle_activity_undo_follow(db.as_ref(), config.as_ref(), id, actor, object).await
            }
            activity::Activity::Like { id, actor, object } => {
                handle_activity_undo_like(db.as_ref(), config.as_ref(), id, actor, object).await
            }
            activity::Activity::Announce { actor, object, .. } => {
                handle_activity_undo_announce(db.as_ref(), config.as_ref(), actor, object).await
            }
            _ => (),
        },
        activity::Activity::Update { object, .. } => match *object {
            activity::Activity::Person {
                id,
                following,
                followers,
                inbox,
                outbox,
                endpoints,
                preferred_username,
                name,
                summary,
                url,
                tag,
                public_key,
                icon,
                image,
            } => {
                handle_activity_update_person(
                    db.as_ref(),
                    config.as_ref(),
                    id,
                    following,
                    followers,
                    inbox,
                    outbox,
                    endpoints,
                    preferred_username,
                    name,
                    summary,
                    url,
                    tag,
                    public_key,
                    icon,
                    image,
                )
                .await
            }
            _ => (),
        },
        _ => (),
    }
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct UserRequest {
    name: String,
}

pub async fn user(
    req: web::Path<UserRequest>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let account = match Account::find()
        .filter(
            Condition::all()
                .add(account::Column::Username.eq(req.name.clone()))
                .add(account::Column::Domain.is_null()),
        )
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(a) => a,
        None => return HttpResponse::NotFound().finish(),
    };

    let person = activity::Activity::from(account);
    HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .json(activity::WithContext::new(person))
}

pub fn naive_date_time_to_rfc3339(dt: chrono::NaiveDateTime) -> String {
    chrono::DateTime::<chrono::Utc>::from_utc(dt, chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
