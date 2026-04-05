use axum::{
    extract::{State, Extension},
    Json,
    http::StatusCode,
};
use crate::{
    config::state::AppState,
    utils::jwt::Claims,
    utils::response::{error,success,success_with_message}
};
use serde_json::json;
use uuid::Uuid;

pub async fn get_dashboard(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>  {

    let user_id = match claims.sub.parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => return Err((
            StatusCode::UNAUTHORIZED,
            error("Invalid token").to_string()
        )),
    };

    let result = async {
        // total income
        let total_income: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(amount) FROM transactions WHERE user_id = $1 AND type = 'income'"
        )
        .bind(user_id)
        .fetch_one(&state.db)
        .await?;

        // total expense
        let total_expense: (Option<f64>,) = sqlx::query_as(
            "SELECT SUM(amount) FROM transactions WHERE user_id = $1 AND type = 'expense'"
        )
        .bind(user_id)
        .fetch_one(&state.db)
        .await?;

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
        .await?;

        let categories: Vec<_> = category_data
        .into_iter()
        .map(|row| {
            json!({
                "category": row.category,
                "total": row.total.unwrap_or(0.0)
            })
        })
        .collect();

        Ok::<_, sqlx::Error>((income, expense, net_balance, categories))

    }
    .await;

    match result {
        Ok((income, expense, net_balance, categories))=> Ok(Json(success(json!({
            "total_income": income,
            "total_expense": expense,
            "net_balance": net_balance,
            "categories": categories    
        })))),

        Err(e)=>{
            println!("DB error (get_dashboard): {:?}", e);

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                error("Failed to fetch dashboard data").to_string()
            ))
        }
    }
}