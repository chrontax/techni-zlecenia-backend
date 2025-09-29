use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};

use lettre::message::{Message, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

use serde::Deserialize;

use crate::{
    auth::Claims,
    db::{postgres::PostgresDb, Db, OfferInput, Order},
    AppState,
};

pub fn router() -> Router<AppState<PostgresDb>> {
    Router::new()
        .route("/offer", post(create_offer_handler))
        .route("/offers/user/{id}", get(get_offers_by_user_handler))
        .route("/offers/order/{id}", get(get_offers_by_order_handler))
        .route("/offers/{id}", post(update_offer_status_handler))
        .route("/offers/{id}", delete(delete_offer_handler))
        .route("/offers/{id}", get(get_offer_handler))
}

async fn create_offer_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Json(offer): Json<OfferInput>,
) -> impl IntoResponse {
    match db.create_offer(offer, claims.sub).await {
        Ok(offer) => {
            let who = db.get_user_by_id(offer.user_id).await.unwrap().unwrap();
            dbg!(&who.username);
            let what = db.get_order_by_id(offer.order_id).await.unwrap().unwrap();
            let orderer = db.get_user_by_id(what.user_id).await.unwrap().unwrap();
            send_email(&orderer.email, &who.username, &what.order_name).await;
            (StatusCode::CREATED, Json(offer)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn send_email(recipient: &str, who: &str, what: &str) {
    // Build the email
    let email = Message::builder()
        .from(
            "techni zamowienia <technizamowienia@gmail.com>"
                .parse()
                .unwrap(),
        )
        .to(recipient.parse().unwrap())
        .subject("Ktos odpowiedzial na twoje ogloszenie")
        .singlepart(
            SinglePart::builder()
                .header(lettre::message::header::ContentType::TEXT_HTML)
                .body(format!(
                    "{} odpowiedzial na twoje zgloszenie: {}",
                    who, what
                )),
        )
        .unwrap();

    // SMTP credentials (use your Gmail address + App Password)
    let creds = Credentials::new(
        "technizamowienia".to_string(),
        "znfbjezwxvremoln".to_string(),
    );
    dbg!(recipient);

    // Open a remote connection to Gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
}

async fn get_offers_by_user_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_offers_by_user_id(id).await {
        Ok(offers) => (StatusCode::OK, Json(offers)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_offers_by_order_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_offers_by_order_id(id).await {
        Ok(offers) => (StatusCode::OK, Json(offers)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[derive(Deserialize)]
struct OfferUpdateBody {
    status: String,
}

async fn update_offer_status_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
    Json(OfferUpdateBody { status }): Json<OfferUpdateBody>,
) -> impl IntoResponse {
    let offer = match db.get_offer_by_id(id).await {
        Ok(Some(offer)) => offer,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    let order = match db.get_order_by_id(offer.order_id).await {
        Ok(Some(order)) => order,
        Ok(None) => unreachable!(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if order.user_id != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.update_offer_status(id, &status).await {
        Ok(offer) => (StatusCode::OK, Json(offer)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn delete_offer_handler<D: Db>(
    claims: Claims,
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    let offer = match db.get_offer_by_id(id).await {
        Ok(Some(offer)) => offer,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    };
    if offer.user_id != claims.sub {
        return StatusCode::FORBIDDEN.into_response();
    }
    match db.delete_offer(id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_offer_handler<D: Db>(
    State(AppState { db }): State<AppState<D>>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    match db.get_offer_by_id(id).await {
        Ok(Some(offer)) => (StatusCode::OK, Json(offer)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
