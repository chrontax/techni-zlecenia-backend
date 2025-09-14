use serde::Deserialize;

pub struct User {
    pub user_id: usize,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: u64,
}

pub struct UserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct Order {
    pub order_id: usize,
    pub user_id: usize,
    pub order_name: String,
    pub order_desc: String,
    pub price: f64,
    pub image_urls: Vec<String>,
    pub created_at: u64,
}

pub struct OrderInput {
    pub order_name: String,
    pub order_desc: String,
    pub price: f64,
    pub image_urls: Vec<String>,
}

pub struct Offer {
    pub offer_id: usize,
    pub order_id: usize,
    pub user_id: usize,
    pub status: String,
    pub created_at: u64,
}

pub struct OfferInput {
    pub order_id: usize,
}

#[derive(Deserialize)]
pub struct Message {
    pub message_id: usize,
    pub sender_id: usize,
    pub receiver_id: usize,
    pub content: String,
    pub sent_at: u64,
}

pub struct MessageInput {
    pub sender_id: usize,
    pub receiver_id: usize,
    pub content: String,
}

pub struct Review {
    pub review_id: usize,
    pub user_reviewed: usize,
    pub user_reviewing: usize,
    pub rating: u8,
    pub content: String,
    pub created_at: u64,
}

pub struct ReviewInput {
    pub user_reviewed: usize,
    pub user_reviewing: usize,
    pub rating: u8,
    pub content: String,
}
