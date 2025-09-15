use axum::{response::IntoResponse, routing::any, Router};

use crate::{auth::create_jwt, chat::ws_handler, db::postgres::PostgresDb, AppState};

pub fn router() -> Router<AppState<PostgresDb>> {
    Router::new().route("/listen", any(ws_handler))
}
