use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    auth::{create_jwt, Claims},
    db::{postgres::PostgresDb, Db, UserInput},
    routes::SearchQuery,
    AppState,
};

pub fn router() -> Router<AppState<PostgresDb>> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        .route("/search", get(search_handler))
        .route("/{id}", get(get_user_handler))
        .route("/{id}", post(update_user_handler))
        .route("/{id}", delete(delete_user_handler))
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String,
}

async fn login_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Json(login): Json<LoginInput>,
) -> impl IntoResponse {
    let user = match db.get_user_by_username(&login.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if Argon2::default()
        .verify_password(
            login.password.as_bytes(),
            &PasswordHash::new(&user.password_hash).unwrap(),
        )
        .is_ok()
    {
        (StatusCode::OK, create_jwt(user.user_id)).into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn register_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Json(mut user): Json<UserInput>,
) -> impl IntoResponse {
    user.password = Argon2::default()
        .hash_password(user.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();
    match db.create_user(user).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn search_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    match db.search_users(&query.query).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_user_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_user_by_id(id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn update_user_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
    Json(user): Json<UserInput>,
) -> impl IntoResponse {
    if claims.sub != id {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.update_user(id, user).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn delete_user_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    if claims.sub != id {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.delete_user(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
