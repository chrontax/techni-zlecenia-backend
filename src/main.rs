use std::time::Duration;

use axum::Router;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::{
    auth::init_keys,
    db::{postgres::PostgresDb, Db},
    routes::{messages, offers, orders, reviews, user},
};

mod auth;
mod chat;
mod db;
mod routes;

#[derive(Clone)]
struct AppState<T: Db> {
    pub db: T,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    init_keys(secrets.get("JWT_SECRET").unwrap().as_bytes());

    let router = Router::new()
        .merge(messages::router())
        .merge(offers::router())
        .merge(orders::router())
        .merge(reviews::router())
        .nest("/user", user::router())
        .layer(CorsLayer::permissive().max_age(Duration::from_secs(60) * 60))
        .with_state(AppState {
            db: PostgresDb::new(pool).await,
        });

    Ok(router.into())
}
