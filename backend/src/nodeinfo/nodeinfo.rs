use actix_web::{web, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Software {
    name: String,
    version: String,
}

#[derive(Serialize)]
struct UsersUsage {
    total: u32,
    #[serde(rename(serialize = "activeMonth"))]
    active_month: u32,
    #[serde(rename(serialize = "activeHalfyear"))]
    active_halfyear: u32,
}

#[derive(Serialize)]
struct Usage {
    users: UsersUsage,
    #[serde(rename(serialize = "localPosts"))]
    local_posts: u32,
}

#[derive(Serialize)]
struct Services {
    outbound: Vec<String>,
    inbound: Vec<String>,
}

#[derive(Serialize)]
struct Metadata {}

#[derive(Serialize)]
struct NodeInfo {
    version: String,
    software: Software,
    protocols: Vec<String>,
    services: Services,
    usage: Usage,
    #[serde(rename(serialize = "openRegistrations"))]
    open_registrations: bool,
    metadata: Metadata,
}

pub async fn index() -> impl Responder {
    let node_info = NodeInfo {
        version: String::from("2.0"),
        software: Software {
            name: String::from("bozudon"),
            version: String::from("0.1.0"),
        },
        protocols: vec![String::from("activitypub")],
        services: Services {
            outbound: vec![],
            inbound: vec![],
        },
        usage: Usage {
            users: UsersUsage {
                total: 1,
                active_month: 1,
                active_halfyear: 1,
            },
            local_posts: 0,
        },
        open_registrations: true,
        metadata: Metadata {},
    };

    web::Json(node_info)
}
