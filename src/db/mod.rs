pub mod model;
pub use model::*;
pub mod postgres;

pub trait Db {
    type MsgListner: MsgListner;

    async fn create_user(&self, user: UserInput) -> Result<User, String>;
    async fn get_user_by_id(&self, user_id: usize) -> Result<Option<User>, String>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String>;
    async fn search_users(&self, query: &str) -> Result<Vec<User>, String>;
    async fn update_user(&self, user_id: usize, user: UserInput) -> Result<User, String>;
    async fn delete_user(&self, user_id: usize) -> Result<(), String>;

    async fn create_order(&self, order: OrderInput, user_id: usize) -> Result<Order, String>;
    async fn get_order_by_id(&self, order_id: usize) -> Result<Option<Order>, String>;
    async fn get_orders_by_user_id(&self, user_id: usize) -> Result<Vec<Order>, String>;
    async fn search_orders(&self, query: &str) -> Result<Vec<Order>, String>;
    async fn update_order(&self, order_id: usize, order: OrderInput) -> Result<Order, String>;
    async fn delete_order(&self, order_id: usize) -> Result<(), String>;

    async fn create_offer(&self, offer: OfferInput, user_id: usize) -> Result<Offer, String>;
    async fn get_offer_by_id(&self, offer_id: usize) -> Result<Option<Offer>, String>;
    async fn get_offers_by_user_id(&self, user_id: usize) -> Result<Vec<Offer>, String>;
    async fn get_offers_by_order_id(&self, order_id: usize) -> Result<Vec<Offer>, String>;
    async fn update_offer_status(&self, offer_id: usize, status: &str) -> Result<Offer, String>;
    async fn delete_offer(&self, offer_id: usize) -> Result<(), String>;

    async fn create_message(&self, message: MessageInput) -> Result<Message, String>;
    async fn update_message(&self, message_id: usize, content: &str) -> Result<Message, String>;
    async fn get_message_by_id(&self, message_id: usize) -> Result<Option<Message>, String>;
    async fn get_messages_between_users(
        &self,
        user1_id: usize,
        user2_id: usize,
    ) -> Result<Vec<Message>, String>;
    async fn get_messaged_users(&self, user_id: usize) -> Result<Vec<User>, String>;
    async fn delete_message(&self, message_id: usize) -> Result<(), String>;
    async fn listen_for_messages(&self, user_id: usize) -> Result<Self::MsgListner, String>;

    async fn create_review(&self, review: ReviewInput) -> Result<Review, String>;
    async fn get_reviews_for_user(&self, user_id: usize) -> Result<Vec<Review>, String>;
    async fn get_reviews_by_user(&self, user_id: usize) -> Result<Vec<Review>, String>;
    async fn get_review_by_id(&self, review_id: usize) -> Result<Option<Review>, String>;
    async fn update_review(
        &self,
        review_id: usize,
        content: &str,
        rating: u8,
    ) -> Result<Review, String>;
    async fn delete_review(&self, review_id: usize) -> Result<(), String>;
}

pub trait MsgListner {
    async fn receive(&mut self) -> Result<Message, String>;
}
