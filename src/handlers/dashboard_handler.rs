use axum::{
    extract::{State, Extension},
    Json,
    http::StatusCode,
};
use crate::{
    config::state::AppState,
    utils::{app_error::AppError, jwt::Claims, response::{success}}
};
use serde_json::{json,Value};
use uuid::Uuid;

pub async fn get_dashboard(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, AppError>  {

    let user_id = match claims.sub.parse::<Uuid>() {
        Ok(id) => id,
        Err(_) => return Err(AppError{
            status: StatusCode::UNAUTHORIZED,
            body: json!({
                "success": false,
                "message": "Invalid Token", 
            })
        }),
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

        //recent activity
        let recent_rows = sqlx::query!(
            r#"
            SELECT id, amount, type, category, created_at
            FROM transactions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 5
            "#,
            user_id
        )
        .fetch_all(&state.db)
        .await?;

        let recent_activity: Vec<_> = recent_rows
        .into_iter()
        .map(|row| {
          json!({
               "id": row.id,
                "amount": row.amount,
                "type": row.r#type,
                "category": row.category,
                "created_at": row.created_at
            })
        })
        .collect();

        //monthly rows
        let monthly_rows = sqlx::query!(
            r#"
            SELECT 
                DATE_TRUNC('month', created_at) as month,
                SUM(amount) as total
            FROM transactions
            WHERE user_id = $1
            GROUP BY month
            ORDER BY month
            "#,
            user_id
        )
        .fetch_all(&state.db)
        .await?;

        let monthly_trends: Vec<_> = monthly_rows
        .into_iter()
        .map(|row| {
            json!({
                "month": row.month,
                "total": row.total.unwrap_or(0.0)
            })
        })
        .collect();

        Ok::<_, sqlx::Error>((
        income,
        expense,
        net_balance,
        categories,
        recent_activity,
        monthly_trends
        ))
    }
    .await;

    match result {
        Ok((income, expense, net_balance, categories, recent_activity, monthly_trends))=> Ok(Json(success(json!({
            "total_income": income,
            "total_expense": expense,
            "net_balance": net_balance,
            "categories": categories,
            "recent_activity": recent_activity,
            "monthly_trends": monthly_trends,
        })))),

        Err(e)=>{
            println!("DB error (get_dashboard): {:?}", e);

            Err(AppError{
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: json!({
                    "success": false,
                    "message": "Failed to fetch dashboard data".to_string()
                })
                    
            })
        }
    }
}