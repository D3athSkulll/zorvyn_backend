use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

#[derive(Clone,Serialize, Deserialize)]
pub struct Claims{
    pub sub: String, //user id
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub fn generate_token(user_id: &str, email: &str, role: &str) -> String{
    let claims = Claims{
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: 2000000000, //temporary
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error>{
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}