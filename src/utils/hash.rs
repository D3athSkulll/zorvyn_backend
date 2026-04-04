use argon2::{
    password_hash::{SaltString, PasswordHasher},
    Argon2
};

use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2= Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    hash
}