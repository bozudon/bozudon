use crate::api_v1::utils::*;
use crate::entity;
use crate::model::prelude::*;
use crate::model::*;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use lazy_static::lazy_static;
use regex::Regex;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::*;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Deserialize)]
pub struct TargetAccountPath {
    pub id: String,
}

#[derive(Deserialize)]
pub struct AccountPostRequest {
    username: String,
    email: String,
    password: String,
    agreement: bool,
    #[allow(dead_code)]
    locale: String,
}

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new("^[a-zA-Z_0-9]{3,30}$").unwrap();
}

pub async fn create_account_post(
    db: web::Data<DatabaseConnection>,
    req: web::Form<AccountPostRequest>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let mut validation_errors = Vec::new();

    if req.password.len() <= 8 {
        validation_errors.push("Password must be longer than 8 chars");
    }

    if !RE_USERNAME.is_match(&req.username) {
        validation_errors.push("Username must contain only letters, numbers and underscores");
    }

    if !req.agreement {
        validation_errors.push("Agreement must be accepted");
    }

    if !validation_errors.is_empty() {
        return HttpResponse::UnprocessableEntity().json(
            [(
                "error",
                format!("Validation failed: {}", validation_errors.join(", ")),
            )]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        );
    }

    let encrypted_password = match bcrypt::hash(req.password.clone(), bcrypt::DEFAULT_COST) {
        Ok(pw) => pw,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let private_key = openssl::rsa::Rsa::generate(2048).unwrap();
    let private_key_pem = private_key.private_key_to_pem().unwrap();
    let public_key_pem = private_key.public_key_to_pem().unwrap();

    match db
        .as_ref()
        .transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let uri = format!("https://{}/users/{}", config.uri, req.username);
                let account = account::ActiveModel {
                    id: NotSet,
                    created_at: NotSet,
                    updated_at: NotSet,
                    username: Set(req.username.clone()),
                    domain: NotSet,
                    display_name: Set(req.username.clone()),
                    note: Set(String::from("")),
                    uri: Set(format!(
                        "https://{}/users/{}",
                        config.uri,
                        req.username.clone()
                    )),
                    url: NotSet,
                    inbox_url: Set(uri.clone() + "/inbox"),
                    outbox_url: Set(uri.clone() + "/outbox"),
                    shared_inbox_url: Set(format!("https://{}/inbox", config.uri)),
                    followers_url: Set(uri.clone() + "/followers"),
                    private_key: Set(Some(
                        str::from_utf8(private_key_pem.as_slice())
                            .unwrap()
                            .to_string(),
                    )),
                    public_key: Set(str::from_utf8(public_key_pem.as_slice())
                        .unwrap()
                        .to_string()),
                }
                .save(txn)
                .await?;

                user::ActiveModel {
                    id: NotSet,
                    created_at: NotSet,
                    updated_at: NotSet,
                    email: Set(req.email.clone()),
                    encrypted_password: Set(encrypted_password),
                    account_id: Set(account.id.unwrap()),
                }
                .save(txn)
                .await?;

                Ok(())
            })
        })
        .await
    {
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
        _ => (),
    };

    HttpResponse::Ok().finish()
}

pub async fn verify_credentials_get(
    db: web::Data<DatabaseConnection>,
    auth: Option<BearerAuth>,
) -> impl Responder {
    // TODO: This endpoint should return a CredentialAccount entity that is strictly different from Account.
    //       The "source" and "role" fields should be returned if requested by the front-end team.

    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let account = match Account::find_by_id(account_id)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(entity::account_model_to_entity(&account, 0, 0, 0)) // FIXME
}

pub async fn account_info_get(
    path: web::Path<TargetAccountPath>,
    auth: Option<BearerAuth>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    match get_authenticated_user(auth, db.as_ref()).await {
        Some(_) => (),
        None => return HttpResponse::Unauthorized().finish(),
    };

    let account_id: i64 = path.id.clone().parse().unwrap();

    match Account::find_by_id(account_id)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(account) => {
            let followers_count = Follow::find()
                .filter(follow::Column::TargetAccountId.eq(account_id))
                .count(db.as_ref())
                .await
                .unwrap();
            let following_count = Follow::find()
                .filter(follow::Column::AccountId.eq(account_id))
                .count(db.as_ref())
                .await
                .unwrap();
            let statuses_count = Status::find()
                .filter(status::Column::AccountId.eq(account_id))
                .count(db.as_ref())
                .await
                .unwrap();

            return HttpResponse::Ok().json(entity::account_model_to_entity(
                &account,
                followers_count as i64,
                following_count as i64,
                statuses_count as i64,
            ));
        }
        None => {
            return HttpResponse::NotFound().json(
                [("error", "Record not found")]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            )
        }
    };
}

#[derive(Deserialize)]
pub struct FieldsAttributesData {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct SourceData {
    pub privacy: Option<String>,
    pub sensitive: Option<bool>,
    pub language: Option<String>,
}

#[derive(Deserialize)]
pub struct AccountInfoUpdateRequest {
    pub display_name: Option<String>,
    pub note: Option<String>,
    //     pub avatar: Option<String>,
    //     pub header: Option<String>,
    //     pub locked: Option<bool>,
    //     pub bot: Option<bool>,
    //     pub discoverable: Option<bool>,
    //     pub fields_attributes: Option<HashMap<String, FieldsAttributesData>>,
    //     pub source: Option<SourceData>,
    // TODO: あとでDB側が対応したら対応する
}

pub async fn account_info_update(
    req: web::Query<AccountInfoUpdateRequest>,
    auth: Option<BearerAuth>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let account = match Account::find_by_id(account_id)
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(account) => account,
        None => return HttpResponse::NotFound().finish(),
    };

    let mut account_new: account::ActiveModel = account.into();

    if req.display_name.is_some() {
        account_new.display_name = Set(req.display_name.clone().unwrap());
    }

    if req.note.is_some() {
        account_new.note = Set(req.note.clone().unwrap());
    }

    let account_new = account_new.update(db.as_ref()).await.unwrap();
    // FIXME
    HttpResponse::Ok().json(entity::account_model_to_entity(&account_new, 0, 0, 0))
}

#[derive(Debug)]
pub struct AccountRelationshipGetRequest {
    pub id: Vec<String>,
}

impl Default for AccountRelationshipGetRequest {
    fn default() -> Self {
        Self { id: Vec::default() }
    }
}

impl<'de> Deserialize<'de> for AccountRelationshipGetRequest {
    fn deserialize<D>(deserializer: D) -> Result<AccountRelationshipGetRequest, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = AccountRelationshipGetRequest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`id`")
            }

            fn visit_map<V>(self, mut map: V) -> Result<AccountRelationshipGetRequest, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut output = AccountRelationshipGetRequest {
                    ..Default::default()
                };
                let mut ids: Vec<String> = Vec::default();
                while let Some(key) = map.next_key()? {
                    match key {
                        "id[]" => ids.push(map.next_value::<String>()?),
                        _ => (),
                    }
                }
                output.id = ids;
                Ok(output)
            }
        }
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

pub async fn account_relationships_get(
    req: web::Query<AccountRelationshipGetRequest>,
    auth: Option<BearerAuth>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let account_id = match get_authenticated_user(auth, db.as_ref()).await {
        Some(user) => user.account_id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut relationships = Vec::new();
    for id in req.id.iter() {
        let target_account_id = match id.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return HttpResponse::BadRequest().finish(),
        };
        let following = match Follow::find()
            .filter(follow::Column::AccountId.eq(account_id))
            .filter(follow::Column::TargetAccountId.eq(target_account_id))
            .one(db.as_ref())
            .await
            .unwrap()
        {
            Some(_) => true,
            None => false,
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

        relationships.push(entity::AccountRelationship {
            id: target_account_id.to_string(),
            following: following,
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

    HttpResponse::Ok().json(relationships)
}
