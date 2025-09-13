use axum::{response::IntoResponse, routing::get, Router};

use crate::{auth::create_jwt, db::postgres::PostgresDb, AppState};

pub fn router() -> Router<AppState<PostgresDb>> {
    Router::new().route("/login", get(login_handler))
}

async fn login_handler() -> impl IntoResponse {
    create_jwt(1)
}
