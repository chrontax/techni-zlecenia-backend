use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{
    multipart::{Form, Part},
    Client, StatusCode,
};
use serde::Deserialize;

use crate::{
    auth::Claims,
    db::{postgres::PostgresDb, Db, OrderInput},
    routes::SearchQuery,
    AppState,
};

pub fn router() -> Router<AppState<PostgresDb>> {
    Router::new()
        .route("/order", post(create_order_handler))
        .route("/orders/user/{id}", get(get_orders_by_user_handler))
        .route("/orders/{id}", get(get_order_handler))
        .route("/orders/{id}", post(update_order_handler))
        .route("/orders/{id}", delete(delete_order_handler))
        .route("/orders/search", get(search_orders_handler))
}

#[derive(Deserialize)]
struct OrderBody {
    order_name: String,
    order_desc: String,
    price: f64,
    images: Vec<String>, // Base 64
}

async fn create_order_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Json(body): Json<OrderBody>,
) -> impl IntoResponse {
    let client = Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap();
    let mut image_urls = Vec::with_capacity(body.images.len());
    for img in body.images {
        let decoded = BASE64_STANDARD.decode(img).unwrap();
        let extension = infer::get(&decoded).unwrap().extension();
        let form = Form::new().text("reqtype", "fileupload").part(
            "fileToUpload",
            Part::stream(decoded).file_name(format!("image.{}", extension)),
        );
        let res = match client
            .post("https://catbox.moe/user/api.php")
            .multipart(form)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };
        image_urls.push(res.text().await.unwrap());
    }

    match db
        .create_order(
            OrderInput {
                order_name: body.order_name,
                order_desc: body.order_desc,
                price: body.price,
                image_urls,
            },
            claims.sub,
        )
        .await
    {
        Ok(order) => (StatusCode::CREATED, Json(order)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_orders_by_user_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_orders_by_user_id(id).await {
        Ok(orders) => (StatusCode::OK, Json(orders)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_order_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_order_by_id(id).await {
        Ok(Some(order)) => (StatusCode::OK, Json(order)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[derive(Deserialize)]
struct OrderUpdateBody {
    order_name: String,
    order_desc: String,
    price: f64,
}

async fn update_order_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
    Json(OrderUpdateBody {
        order_name,
        order_desc,
        price,
    }): Json<OrderUpdateBody>,
) -> impl IntoResponse {
    let order = match db.get_order_by_id(id).await {
        Ok(Some(order)) => order,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if order.user_id != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db
        .update_order(
            id,
            OrderInput {
                order_name,
                order_desc,
                price,
                image_urls: order.image_urls,
            },
        )
        .await
    {
        Ok(order) => (StatusCode::OK, Json(order)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn delete_order_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let order = match db.get_order_by_id(id).await {
        Ok(Some(order)) => order,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if order.user_id != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.delete_order(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn search_orders_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    match db.search_orders(&query.query).await {
        Ok(orders) => (StatusCode::OK, Json(orders)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
