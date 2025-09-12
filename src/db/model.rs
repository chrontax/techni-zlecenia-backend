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

pub struct Offer {
    pub offer_id: usize,
    pub user_id: usize,
    pub offer_name: String,
    pub offer_desc: String,
    pub price: f64,
    pub max_orders: u32,
    pub image_urls: Vec<String>,
    pub created_at: u64,
}

pub struct OfferInput {
    pub offer_name: String,
    pub offer_desc: String,
    pub price: f64,
    pub max_orders: u32,
    pub image_urls: Vec<String>,
}

pub struct Order {
    pub order_id: usize,
    pub offer_id: usize,
    pub user_id: usize,
    pub status: String,
    pub ordered_at: u64,
}

pub struct OrderInput {
    pub offer_id: usize,
    pub user_id: usize,
}

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
