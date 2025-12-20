use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use uuid::Uuid;
use crate::models::Claims;

const JWT_SECRET: &[u8] = b"your-secret-key-change-this-in-production";

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn create_token(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(JWT_SECRET)
    )
}

pub fn verify_token(token: &str) -> Result<Uuid, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;

    Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| {
            jsonwebtoken::errors::ErrorKind::InvalidSubject.into()
        })
}