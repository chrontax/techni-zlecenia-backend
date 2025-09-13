use sqlx::PgPool;

use crate::db::*;

// TODO: Replace with actual implementations

#[derive(Clone)]
pub struct PostgresDb {
    pool: PgPool,
}

impl PostgresDb {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Db for PostgresDb {
    type MsgListner = PostgresMsgListener;

    async fn create_user(&self, user: UserInput) -> Result<User, String> {
        Ok(User {
            user_id: 0,
            username: user.username,
            email: user.email,
            password_hash: "hashed_password".to_string(),
            created_at: 0,
        })
    }

    async fn get_user_by_id(&self, user_id: usize) -> Result<Option<User>, String> {
        Ok(Some(User {
            user_id,
            username: "testuser".to_string(),
            email: "a@b.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: 0,
        }))
    }

    async fn search_users(&self, query: &str) -> Result<Vec<User>, String> {
        Ok(vec![User {
            user_id: 1,
            username: query.to_string(),
            email: "a@b.c".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: 0,
        }])
    }

    async fn update_user(&self, user_id: usize, user: UserInput) -> Result<User, String> {
        Ok(User {
            user_id,
            username: user.username,
            email: user.email,
            password_hash: "hashed_password".to_string(),
            created_at: 0,
        })
    }

    async fn delete_user(&self, user_id: usize) -> Result<(), String> {
        Ok(())
    }

    async fn create_order(&self, order: OrderInput) -> Result<Order, String> {
        Ok(Order {
            order_id: 0,
            user_id: 0,
            order_name: order.order_name,
            order_desc: order.order_desc,
            price: order.price,
            image_urls: order.image_urls,
            created_at: 0,
        })
    }

    async fn get_order_by_id(&self, order_id: usize) -> Result<Option<Order>, String> {
        Ok(Some(Order {
            order_id,
            user_id: 1,
            order_name: "Test order".to_string(),
            order_desc: "This is a test order".to_string(),
            price: 9.99,
            image_urls: vec![],
            created_at: 0,
        }))
    }

    async fn get_orders_by_user_id(&self, user_id: usize) -> Result<Vec<Order>, String> {
        Ok(vec![Order {
            order_id: 1,
            user_id,
            order_name: "Test order".to_string(),
            order_desc: "This is a test order".to_string(),
            price: 9.99,
            image_urls: vec![],
            created_at: 0,
        }])
    }

    async fn search_orders(&self, query: &str) -> Result<Vec<Order>, String> {
        Ok(vec![Order {
            order_id: 1,
            user_id: 1,
            order_name: query.to_string(),
            order_desc: "This is a test order".to_string(),
            price: 9.99,
            image_urls: vec![],
            created_at: 0,
        }])
    }

    async fn update_order(&self, order_id: usize, order: OrderInput) -> Result<Order, String> {
        Ok(Order {
            order_id,
            user_id: 1,
            order_name: order.order_name,
            order_desc: order.order_desc,
            price: order.price,
            image_urls: order.image_urls,
            created_at: 0,
        })
    }

    async fn delete_order(&self, order_id: usize) -> Result<(), String> {
        Ok(())
    }

    async fn create_offer(&self, offer: OfferInput, user_id: usize) -> Result<Offer, String> {
        Ok(Offer {
            offer_id: 0,
            order_id: offer.order_id,
            user_id,
            status: "pending".to_string(),
            created_at: 0,
        })
    }

    async fn get_offer_by_id(&self, offer_id: usize) -> Result<Option<Offer>, String> {
        Ok(Some(Offer {
            offer_id,
            order_id: 1,
            user_id: 1,
            status: "pending".to_string(),
            created_at: 0,
        }))
    }

    async fn get_offers_by_user_id(&self, user_id: usize) -> Result<Vec<Offer>, String> {
        Ok(vec![Offer {
            offer_id: 1,
            order_id: 1,
            user_id,
            status: "pending".to_string(),
            created_at: 0,
        }])
    }

    async fn get_offers_by_order_id(&self, order_id: usize) -> Result<Vec<Offer>, String> {
        Ok(vec![Offer {
            offer_id: 1,
            order_id,
            user_id: 1,
            status: "pending".to_string(),
            created_at: 0,
        }])
    }

    async fn update_offer_status(&self, offer_id: usize, status: &str) -> Result<Offer, String> {
        Ok(Offer {
            offer_id,
            order_id: 1,
            user_id: 1,
            status: status.to_string(),
            created_at: 0,
        })
    }

    async fn delete_offer(&self, offer_id: usize) -> Result<(), String> {
        Ok(())
    }

    async fn create_message(&self, message: MessageInput) -> Result<Message, String> {
        Ok(Message {
            message_id: 0,
            sender_id: message.sender_id,
            receiver_id: message.receiver_id,
            content: message.content,
            sent_at: 0,
        })
    }

    async fn update_message(&self, message_id: usize, content: &str) -> Result<Message, String> {
        Ok(Message {
            message_id,
            sender_id: 1,
            receiver_id: 2,
            content: content.to_string(),
            sent_at: 0,
        })
    }

    async fn get_messages_between_users(
        &self,
        user1_id: usize,
        user2_id: usize,
    ) -> Result<Vec<Message>, String> {
        Ok(vec![Message {
            message_id: 1,
            sender_id: user1_id,
            receiver_id: user2_id,
            content: "Hello".to_string(),
            sent_at: 0,
        }])
    }

    async fn get_messaged_users(&self, user_id: usize) -> Result<Vec<User>, String> {
        Ok(vec![User {
            user_id: 2,
            username: "otheruser".to_string(),
            email: "a@b.c".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: 0,
        }])
    }

    async fn delete_message(&self, message_id: usize) -> Result<(), String> {
        Ok(())
    }

    async fn listen_for_messages(&self, user_id: usize) -> Result<Self::MsgListner, String> {
        Ok(PostgresMsgListener)
    }

    async fn create_review(&self, review: ReviewInput) -> Result<Review, String> {
        Ok(Review {
            review_id: 0,
            user_reviewed: review.user_reviewed,
            user_reviewing: review.user_reviewing,
            content: review.content,
            rating: review.rating,
            created_at: 0,
        })
    }

    async fn get_reviews_for_user(&self, user_id: usize) -> Result<Vec<Review>, String> {
        Ok(vec![Review {
            review_id: 1,
            user_reviewed: user_id,
            user_reviewing: 2,
            content: "Great user!".to_string(),
            rating: 5,
            created_at: 0,
        }])
    }

    async fn get_reviews_by_user(&self, user_id: usize) -> Result<Vec<Review>, String> {
        Ok(vec![Review {
            review_id: 1,
            user_reviewed: 2,
            user_reviewing: user_id,
            content: "Great user!".to_string(),
            rating: 5,
            created_at: 0,
        }])
    }

    async fn update_review(
        &self,
        review_id: usize,
        content: &str,
        rating: u8,
    ) -> Result<Review, String> {
        Ok(Review {
            review_id,
            user_reviewed: 2,
            user_reviewing: 1,
            content: content.to_string(),
            rating,
            created_at: 0,
        })
    }

    async fn delete_review(&self, review_id: usize) -> Result<(), String> {
        Ok(())
    }
}

pub struct PostgresMsgListener;

impl MsgListner for PostgresMsgListener {
    async fn receive(&self) -> Result<Message, String> {
        Ok(Message {
            message_id: 0,
            sender_id: 0,
            receiver_id: 0,
            content: "Not implemented".to_string(),
            sent_at: 0,
        })
    }
}
