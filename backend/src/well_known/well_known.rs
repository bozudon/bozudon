use crate::config::Config;
use crate::model::prelude::*;
use crate::model::*;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use regex::Regex;
use sea_orm::{query::*, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

pub async fn host_meta(config: web::Data<Config>) -> impl Responder {
    let body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
  <Link rel="lrdd" template="https://{}/.well-known/webfinger?resource={{uri}}"/>
</XRD>"#,
        config.uri
    );
    HttpResponse::Ok()
        .content_type("application/xrd+xml")
        .body(body)
}

pub async fn nodeinfo(config: web::Data<Config>) -> impl Responder {
    let body = format!(
        r#"{{"links": [{{"rel": "http://nodeinfo.diaspora.software/ns/schema/2.0", "href": "https://{}/nodeinfo/2.0"}}]}}"#,
        config.uri
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[derive(Deserialize)]
pub struct WebfingerRequest {
    resource: String,
}

#[derive(Serialize)]
pub struct WebfingerLink {
    rel: String,
    #[serde(rename = "type")]
    type_: String,
    href: String,
}

#[derive(Serialize)]
pub struct WebfingerResponse {
    subject: String,
    aliases: Vec<String>,
    links: Vec<WebfingerLink>,
}

fn acct(name: &String, dom: &Option<String>) -> String {
    match dom {
        None => name.clone(),
        Some(dom) => format!("{}@{}", name, dom),
    }
}

pub async fn webfinger(
    db: web::Data<DatabaseConnection>,
    req: web::Query<WebfingerRequest>,
    config: web::Data<crate::config::Config>,
) -> impl Responder {
    let re: Regex = Regex::new(r"^(?:acct:)?([^@]+)@(.+)$").unwrap();
    let queries = re
        .captures_iter(req.resource.as_str())
        .map(|c| {
            let (_, [name, dom]) = c.extract();
            (name.to_string(), dom.to_string())
        })
        .collect::<Vec<(String, String)>>();
    info!("queries: {:?}", queries);
    if queries.len() != 1 {
        return HttpResponse::NotFound().finish();
    }
    let (name, dom) = queries[0].clone();
    if dom != config.uri {
        return HttpResponse::BadRequest().finish();
    }

    // Check if name exists
    match Account::find()
        .filter(
            Condition::all()
                .add(account::Column::Username.eq(name.clone()))
                .add(account::Column::Domain.is_null()),
        )
        .one(db.as_ref())
        .await
        .unwrap()
    {
        Some(_) => (),
        None => return HttpResponse::NotFound().finish(),
    };

    let response = WebfingerResponse {
        subject: format!("acct:{}", acct(&name, &Some(dom))),
        aliases: vec![format!("https://{}/users/{}", config.uri, name)],
        links: vec![WebfingerLink {
            rel: "self".to_string(),
            type_: "application/activity+json".to_string(),
            href: format!("https://{}/users/{}", config.uri, name),
        }],
    };

    HttpResponse::Ok()
        .content_type("application/jrd+json; charset=utf-8")
        .json(response)
}
