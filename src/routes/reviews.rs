use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    auth::Claims,
    db::{postgres::PostgresDb, Db, ReviewInput},
    AppState,
};

pub fn router() -> Router<AppState<PostgresDb>> {
    Router::new()
        .route("/reviews/for/{id}", get(get_reviews_for_user_handler))
        .route("/reviews/by/{id}", get(get_reviews_by_user_handler))
        .route("/reviews/{id}", get(get_review_handler))
        .route("/reviews/{id}", post(update_review_handler))
        .route("/reviews/{id}", delete(delete_review_handler))
        .route("/review", post(create_review_handler))
}

#[derive(Deserialize)]
struct ReviewBody {
    user_id: usize,
    rating: u8,
    content: String,
}

async fn create_review_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Json(review): Json<ReviewBody>,
) -> impl IntoResponse {
    match db
        .create_review(ReviewInput {
            user_reviewed: review.user_id,
            user_reviewing: claims.sub,
            rating: review.rating,
            content: review.content,
        })
        .await
    {
        Ok(review) => (StatusCode::CREATED, Json(review)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_reviews_for_user_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_reviews_for_user(id).await {
        Ok(reviews) => (StatusCode::OK, Json(reviews)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_reviews_by_user_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_reviews_by_user(id).await {
        Ok(reviews) => (StatusCode::OK, Json(reviews)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_review_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_review_by_id(id).await {
        Ok(Some(review)) => (StatusCode::OK, Json(review)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[derive(Deserialize)]
struct ReviewUpdateBody {
    rating: u8,
    content: String,
}

async fn update_review_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
    Json(ReviewUpdateBody { rating, content }): Json<ReviewUpdateBody>,
) -> impl IntoResponse {
    let review = match db.get_review_by_id(id).await {
        Ok(Some(review)) => review,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if review.user_reviewing != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.update_review(id, &content, rating).await {
        Ok(review) => (StatusCode::OK, Json(review)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn delete_review_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let review = match db.get_review_by_id(id).await {
        Ok(Some(review)) => review,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if review.user_reviewing != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.delete_review(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
