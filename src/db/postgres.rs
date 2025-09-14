use sqlx::{
    migrate,
    postgres::{PgListener, PgRow},
    query, PgPool, Row,
};

use crate::db::*;

#[derive(Clone)]
pub struct PostgresDb {
    pool: PgPool,
}

impl PostgresDb {
    pub async fn new(pool: PgPool) -> Self {
        migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Self { pool }
    }
}

impl Db for PostgresDb {
    type MsgListner = PostgresMsgListener;

    async fn create_user(&self, user: UserInput) -> Result<User, String> {
        query("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)")
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password) // Assumes the caller has hashed the password
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.get_user_by_username(&user.username)
            .await
            .map(|opt| opt.expect("User just created should exist"))
    }

    async fn get_user_by_id(&self, user_id: usize) -> Result<Option<User>, String> {
        Ok(query("SELECT user_id, username, email, password_hash, created_at FROM users WHERE user_id = $1")
            .bind(user_id as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .map(|row| row.into()))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, String> {
        Ok(query("SELECT user_id, username, email, password_hash, created_at FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .map(|row| row.into()))
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        Ok(query("SELECT user_id, username, email, password_hash, created_at FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .map(|row| row.into()))
    }

    async fn search_users(&self, query: &str) -> Result<Vec<User>, String> {
        Ok(sqlx::query("SELECT user_id, username, email, password_hash, created_at FROM users ORDER BY SILIMILARITY(username, $1) DESC LIMIT 10")
            .bind(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<_>>())
    }

    async fn update_user(&self, user_id: usize, user: UserInput) -> Result<User, String> {
        query("UPDATE users SET username = $1, email = $2, password_hash = $3 WHERE user_id = $4")
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password) // Assumes the caller has hashed the password
            .bind(user_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.get_user_by_id(user_id)
            .await
            .map(|opt| opt.expect("User just updated should exist"))
    }

    async fn delete_user(&self, user_id: usize) -> Result<(), String> {
        query("DELETE FROM users WHERE user_id = $1")
            .bind(user_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create_order(&self, order: OrderInput, user_id: usize) -> Result<(), String> {
        query("INSERT INTO orders (user_id, order_name, order_desc, price, image_urls) VALUES ($1, $2, $3, $4, $5)")
            .bind(user_id as i64)
            .bind(&order.order_name)
            .bind(&order.order_desc)
            .bind(order.price)
            .bind(&order.image_urls)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_order_by_id(&self, order_id: usize) -> Result<Option<Order>, String> {
        Ok(query("SELECT order_id, user_id, order_name, order_desc, price, image_urls, created_at FROM orders WHERE order_id = $1")
            .bind(order_id as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .map(|row| row.into()))
    }

    async fn get_orders_by_user_id(&self, user_id: usize) -> Result<Vec<Order>, String> {
        Ok(query("SELECT order_id, user_id, order_name, order_desc, price, image_urls, created_at FROM orders WHERE user_id = $1")
            .bind(user_id as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<_>>())
    }

    async fn search_orders(&self, query: &str) -> Result<Vec<Order>, String> {
        Ok(sqlx::query("SELECT order_id, user_id, order_name, order_desc, price, image_urls, created_at FROM orders ORDER BY SIMILARITY(order_name, $1) DESC LIMIT 10")
            .bind(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<_>>())
    }

    async fn update_order(&self, order_id: usize, order: OrderInput) -> Result<Order, String> {
        query("UPDATE orders SET order_name = $1, order_desc = $2, price = $3, image_urls = $4 WHERE order_id = $5")
            .bind(&order.order_name)
            .bind(&order.order_desc)
            .bind(order.price)
            .bind(&order.image_urls)
            .bind(order_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.get_order_by_id(order_id)
            .await
            .map(|opt| opt.expect("Order just updated should exist"))
    }

    async fn delete_order(&self, order_id: usize) -> Result<(), String> {
        query("DELETE FROM orders WHERE order_id = $1")
            .bind(order_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create_offer(&self, offer: OfferInput, user_id: usize) -> Result<(), String> {
        query("INSERT INTO offers (order_id, user_id, status) VALUES ($1, $2, 'pending')")
            .bind(offer.order_id as i64)
            .bind(user_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_offer_by_id(&self, offer_id: usize) -> Result<Option<Offer>, String> {
        Ok(query("SELECT offer_id, order_id, user_id, status, created_at FROM offers WHERE offer_id = $1")
            .bind(offer_id as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .map(|row| row.into()))
    }

    async fn get_offers_by_user_id(&self, user_id: usize) -> Result<Vec<Offer>, String> {
        Ok(query(
            "SELECT offer_id, order_id, user_id, status, created_at FROM offers WHERE user_id = $1",
        )
        .bind(user_id as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|row| row.into())
        .collect::<Vec<_>>())
    }

    async fn get_offers_by_order_id(&self, order_id: usize) -> Result<Vec<Offer>, String> {
        Ok(query(
            "SELECT offer_id, order_id, user_id, status, created_at FROM offers WHERE order_id = $1",
        )
        .bind(order_id as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|row| row.into())
        .collect::<Vec<_>>())
    }

    async fn update_offer_status(&self, offer_id: usize, status: &str) -> Result<Offer, String> {
        query("UPDATE offers SET status = $1 WHERE offer_id = $2")
            .bind(status)
            .bind(offer_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.get_offer_by_id(offer_id)
            .await
            .map(|opt| opt.expect("Offer just updated should exist"))
    }

    async fn delete_offer(&self, offer_id: usize) -> Result<(), String> {
        query("DELETE FROM offers WHERE offer_id = $1")
            .bind(offer_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create_message(&self, message: MessageInput) -> Result<(), String> {
        query("INSERT INTO messages (sender_id, receiver_id, content) VALUES ($1, $2, $3)")
            .bind(message.sender_id as i64)
            .bind(message.receiver_id as i64)
            .bind(&message.content)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn update_message(&self, message_id: usize, content: &str) -> Result<Message, String> {
        query("UPDATE messages SET content = $1 WHERE message_id = $2")
            .bind(content)
            .bind(message_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.get_message_by_id(message_id)
            .await
            .map(|opt| opt.expect("Message just updated should exist"))
    }

    async fn get_message_by_id(&self, message_id: usize) -> Result<Option<Message>, String> {
        Ok(query("SELECT message_id, sender_id, receiver_id, content, sent_at FROM messages WHERE message_id = $1")
            .bind(message_id as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .map(|row| row.into()))
    }

    async fn get_messages_between_users(
        &self,
        user1_id: usize,
        user2_id: usize,
    ) -> Result<Vec<Message>, String> {
        Ok(query("SELECT message_id, sender_id, receiver_id, content, sent_at FROM messages WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1) ORDER BY sent_at ASC")
            .bind(user1_id as i64)
            .bind(user2_id as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<_>>())
    }

    async fn get_messaged_users(&self, user_id: usize) -> Result<Vec<User>, String> {
        Ok(query("SELECT DISTINCT u.user_id, u.username, u.email, u.password_hash, u.created_at FROM users u JOIN messages m ON (u.user_id = m.sender_id OR u.user_id = m.receiver_id) WHERE m.sender_id = $1 OR m.receiver_id = $1")
            .bind(user_id as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<_>>())
    }

    async fn delete_message(&self, message_id: usize) -> Result<(), String> {
        query("DELETE FROM messages WHERE message_id = $1")
            .bind(message_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn listen_for_messages(&self, user_id: usize) -> Result<Self::MsgListner, String> {
        let mut listener = PgListener::connect_with(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        listener
            .listen(&user_id.to_string())
            .await
            .map_err(|e| e.to_string())?;
        Ok(PostgresMsgListener { listener })
    }

    async fn create_review(&self, review: ReviewInput) -> Result<(), String> {
        query("INSERT INTO reviews (user_reviewed, user_reviewing, content, rating) VALUES ($1, $2, $3, $4)")
            .bind(review.user_reviewed as i64)
            .bind(review.user_reviewing as i64)
            .bind(&review.content)
            .bind(review.rating as i16)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_reviews_for_user(&self, user_id: usize) -> Result<Vec<Review>, String> {
        Ok(query("SELECT review_id, user_reviewed, user_reviewing, content, rating, created_at FROM reviews WHERE user_reviewed = $1")
            .bind(user_id as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<_>>())
    }

    async fn get_reviews_by_user(&self, user_id: usize) -> Result<Vec<Review>, String> {
        Ok(query("SELECT review_id, user_reviewed, user_reviewing, content, rating, created_at FROM reviews WHERE user_reviewing = $1")
            .bind(user_id as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<_>>())
    }

    async fn get_review_by_id(&self, review_id: usize) -> Result<Option<Review>, String> {
        Ok(query("SELECT review_id, user_reviewed, user_reviewing, content, rating, created_at FROM reviews WHERE review_id = $1")
            .bind(review_id as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .map(|row| row.into()))
    }

    async fn update_review(
        &self,
        review_id: usize,
        content: &str,
        rating: u8,
    ) -> Result<Review, String> {
        query("UPDATE reviews SET content = $1, rating = $2 WHERE review_id = $3")
            .bind(content)
            .bind(rating as i16)
            .bind(review_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        self.get_reviews_by_user(review_id).await.map(|opt| {
            opt.into_iter()
                .find(|r| r.review_id == review_id)
                .expect("Review just updated should exist")
        })
    }

    async fn delete_review(&self, review_id: usize) -> Result<(), String> {
        query("DELETE FROM reviews WHERE review_id = $1")
            .bind(review_id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub struct PostgresMsgListener {
    listener: PgListener,
}

impl MsgListner for PostgresMsgListener {
    async fn receive(&mut self) -> Result<Message, String> {
        let notification = self.listener.recv().await.map_err(|e| e.to_string())?;
        serde_json::from_str::<Message>(notification.payload()).map_err(|e| e.to_string())
    }
}

impl From<PgRow> for Message {
    fn from(row: PgRow) -> Self {
        Message {
            message_id: row.get::<i64, _>("message_id") as usize,
            sender_id: row.get::<i64, _>("sender_id") as usize,
            receiver_id: row.get::<i64, _>("receiver_id") as usize,
            content: row.get("content"),
            sent_at: row.get::<i64, _>("sent_at") as u64,
        }
    }
}

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self {
        User {
            user_id: row.get::<i64, _>("user_id") as usize,
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get::<i64, _>("created_at") as u64,
        }
    }
}

impl From<PgRow> for Order {
    fn from(row: PgRow) -> Self {
        Order {
            order_id: row.get::<i64, _>("order_id") as usize,
            user_id: row.get::<i64, _>("user_id") as usize,
            order_name: row.get("order_name"),
            order_desc: row.get("order_desc"),
            price: row.get("price"),
            image_urls: row.get("image_urls"),
            created_at: row.get::<i64, _>("created_at") as u64,
        }
    }
}

impl From<PgRow> for Offer {
    fn from(row: PgRow) -> Self {
        Offer {
            offer_id: row.get::<i64, _>("offer_id") as usize,
            order_id: row.get::<i64, _>("order_id") as usize,
            user_id: row.get::<i64, _>("user_id") as usize,
            status: row.get("status"),
            created_at: row.get::<i64, _>("created_at") as u64,
        }
    }
}

impl From<PgRow> for Review {
    fn from(row: PgRow) -> Self {
        Review {
            review_id: row.get::<i64, _>("review_id") as usize,
            user_reviewed: row.get::<i64, _>("user_reviewed") as usize,
            user_reviewing: row.get::<i64, _>("user_reviewing") as usize,
            content: row.get("content"),
            rating: row.get::<i16, _>("rating") as u8,
            created_at: row.get::<i64, _>("created_at") as u64,
        }
    }
}
