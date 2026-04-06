use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::{User, PublicUser};

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

pub async fn get_all_users(pool: &PgPool)
    -> Result<Vec<PublicUser>, sqlx::Error>{
        let users = sqlx::query_as::<_, PublicUser>(
            "SELECT id, name, email, role, created_at FROM users"
        )
        .fetch_all(pool)
        .await?;
    
    Ok(users)
}

pub async fn update_user_role(
    pool: &PgPool,
    user_id: Uuid,
    role: &str,
) -> Result<PublicUser, sqlx::Error> {

    let user = sqlx::query_as::<_, PublicUser>(
        r#"
        UPDATE users
        SET role = $1
        WHERE id = $2
        RETURNING id, name, email, role, created_at
        "#
    )
    .bind(role)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    match user {
        Some(u) => Ok(u),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn delete_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {

    let result = sqlx::query(
        "DELETE FROM users WHERE id = $1"
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}