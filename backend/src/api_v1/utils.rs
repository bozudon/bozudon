use crate::model::prelude::*;
use crate::model::*;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, QueryFilter};

pub async fn get_authenticated_user(
    auth: Option<BearerAuth>,
    db: &DatabaseConnection,
) -> Option<user::Model> {
    match auth {
        Some(auth) => {
            match OauthAccessToken::find()
                .find_also_related(User)
                .filter(oauth_access_token::Column::Token.eq(auth.token()))
                .one(db)
                .await
                .unwrap()
            {
                Some((_, user)) => {
                    let user = user.unwrap();
                    Some(user)
                }
                None => None,
            }
        }
        None => None,
    }
}

pub async fn get_authenticated_app(
    auth: Option<BearerAuth>,
    db: &DatabaseConnection,
) -> Option<app::Model> {
    match auth {
        Some(auth) => {
            match OauthAccessToken::find()
                .find_also_related(App)
                .filter(oauth_access_token::Column::Token.eq(auth.token()))
                .one(db)
                .await
                .unwrap()
            {
                Some((_, app)) => {
                    let app = app.unwrap();
                    Some(app)
                }
                None => None,
            }
        }
        None => None,
    }
}
