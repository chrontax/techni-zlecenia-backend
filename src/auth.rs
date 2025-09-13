use std::fmt::Debug;

use axum::{
    extract::{FromRequestParts, Query},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

static KEYS: OnceCell<Keys> = OnceCell::new();

struct Keys {
    enc: EncodingKey,
    dec: DecodingKey,
}

impl Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keys").finish()
    }
}

pub fn init_keys(secret: &[u8]) {
    let keys = Keys {
        enc: EncodingKey::from_secret(secret),
        dec: DecodingKey::from_secret(secret),
    };
    KEYS.set(keys).expect("Keys already initialized");
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: usize,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Deserialize)]
struct Token {
    token: String,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token;
        if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            token = bearer.token().to_string();
        } else if let Ok(Query(Token { token: t })) = parts.extract::<Query<Token>>().await {
            token = t;
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
        decode::<Claims>(
            &token,
            &KEYS.get().expect("Keys not initialized").dec,
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| StatusCode::UNAUTHORIZED)
    }
}
