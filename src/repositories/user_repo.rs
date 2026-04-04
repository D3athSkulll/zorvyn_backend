use sqlx::PgPool;
use crate::models::user::User;

pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
    role: &str,
) -> Result<User, sqlx::Error>{
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, email, password_hash, role)
        VALUES ($1,$2,$3,$4)
        RETURNING *
        "#
    )
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<User, sqlx::Error>{
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(user)
}