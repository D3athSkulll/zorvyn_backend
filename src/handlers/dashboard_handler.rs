use axum::{
    extract::{State, Extension},
    Json,
};
use crate::{
    config::state::AppState,
    utils::jwt::Claims,
};
use serde_json::json;
use uuid::Uuid;

pub async fn get_dashboard(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Json<serde_json::Value> {

    let user_id: Uuid = claims.sub.parse().unwrap();

    // total income
    let total_income: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(amount) FROM transactions WHERE user_id = $1 AND type = 'income'"
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .unwrap();

    // total expense
    let total_expense: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(amount) FROM transactions WHERE user_id = $1 AND type = 'expense'"
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let income = total_income.0.unwrap_or(0.0);
    let expense = total_expense.0.unwrap_or(0.0);

    let net_balance = income - expense;

    // category-wise aggregation
    let category_data = sqlx::query!(
        r#"
        SELECT category, SUM(amount) as total
        FROM transactions
        WHERE user_id = $1
        GROUP BY category
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    let categories: Vec<_> = category_data
    .into_iter()
    .map(|row| {
        json!({
            "category": row.category,
            "total": row.total.unwrap_or(0.0)
        })
    })
    .collect();

    Json(json!({
    "success": true,
    "data": {
        "total_income": income,
        "total_expense": expense,
        "net_balance": net_balance,
        "categories": categories
    }
    }))
}