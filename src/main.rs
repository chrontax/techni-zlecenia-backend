use axum::{routing::any, Router};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

use crate::{
    auth::init_keys,
    chat::ws_handler,
    db::{postgres::PostgresDb, Db},
};

mod auth;
mod chat;
mod db;

#[derive(Clone)]
struct State<T: Db> {
    pub db: T,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    init_keys(secrets.get("JWT_SECRET").unwrap().as_bytes());

    let router = Router::new()
        .route("/ws", any(ws_handler))
        .with_state(State {
            db: PostgresDb::new(pool),
        });

    Ok(router.into())
}
