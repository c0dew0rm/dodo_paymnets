use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str, secret_key: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
        + 3600;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).unwrap()
}

pub fn decode_jwt(token: &str, secret_key: &str) -> TokenData<Claims> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::default()).unwrap()
}
