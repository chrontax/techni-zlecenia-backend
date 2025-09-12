use axum::Router;
use sqlx::PgPool;

use crate::db::{postgres::PostgresDb, Db};

mod db;

#[derive(Clone)]
struct State<T: Db> {
    pub db: T,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().with_state(State {
        db: PostgresDb::new(pool),
    });

    Ok(router.into())
}
