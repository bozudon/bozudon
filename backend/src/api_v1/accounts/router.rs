use crate::api_v1::accounts;
use actix_web::{web, Scope};

pub fn router() -> Scope {
    web::scope("/accounts")
        .route("", web::post().to(accounts::accounts::create_account_post))
        .route(
            "/verify_credentials",
            web::get().to(accounts::accounts::verify_credentials_get),
        )
        .route(
            "/update_credentials",
            web::patch().to(accounts::accounts::account_info_update),
        )
        .route(
            "/{id}/unfollow",
            web::post().to(accounts::follow::account_unfollow_post),
        )
        .route(
            "/{id}/follow",
            web::post().to(accounts::follow::account_follow_post),
        )
        .route(
            "/{id}/followers",
            web::get().to(accounts::follow::account_followers_get),
        )
        .route(
            "/{id}/following",
            web::get().to(accounts::follow::account_following_get),
        )
        .route(
            "/{id}/statuses",
            web::get().to(accounts::statuses::account_statuses_get),
        )
        .route(
            "/relationships",
            web::get().to(accounts::accounts::account_relationships_get),
        )
        .route("/{id}", web::get().to(accounts::accounts::account_info_get)) // これを最後に持ってこないとその下に書いたルーティングがおかしくなる
}
