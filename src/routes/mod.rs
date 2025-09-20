use serde::Deserialize;

pub mod messages;
pub mod offers;
pub mod orders;
pub mod reviews;
pub mod user;

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}
