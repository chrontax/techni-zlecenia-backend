use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{any, delete, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    auth::Claims,
    chat::ws_handler,
    db::{postgres::PostgresDb, Db, MessageInput},
    AppState,
};

pub fn router() -> Router<AppState<PostgresDb>> {
    Router::new()
        .route("/messages/listen", any(ws_handler))
        .route("/message", post(send_message_handler))
        .route("/message/{id}", delete(delete_message_handler))
        .route("/message/{id}", post(update_message_handler))
}

#[derive(Deserialize)]
struct MessageBody {
    receiver_id: usize,
    content: String,
}

async fn send_message_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Json(msg): Json<MessageBody>,
) -> impl IntoResponse {
    match db
        .create_message(MessageInput {
            sender_id: claims.sub,
            receiver_id: msg.receiver_id,
            content: msg.content,
        })
        .await
    {
        Ok(msg) => (StatusCode::CREATED, Json(msg)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn delete_message_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let msg = match db.get_message_by_id(id).await {
        Ok(Some(msg)) => msg,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if msg.sender_id != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.delete_message(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[derive(Deserialize)]
struct MessageUpdateBody {
    content: String,
}

async fn update_message_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
    Json(MessageUpdateBody { content }): Json<MessageUpdateBody>,
) -> impl IntoResponse {
    let msg = match db.get_message_by_id(id).await {
        Ok(Some(msg)) => msg,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if msg.sender_id != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.update_message(id, &content).await {
        Ok(msg) => (StatusCode::OK, Json(msg)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
