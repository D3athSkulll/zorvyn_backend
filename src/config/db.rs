use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

pub async fn connect_db()-> sqlx::PgPool{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("{}", db_url);

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}