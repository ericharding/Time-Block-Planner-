use crate::{
    auth::AuthUser,
    handlers::auth::AppError,
    models::{Column, CreateColumnRequest},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

pub async fn create_column(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateColumnRequest>,
) -> Result<Json<Column>, AppError> {
    // Verify plan belongs to user
    let plan = sqlx::query!("SELECT id FROM plans WHERE id = ? AND user_id = ?", payload.plan_id, auth_user.0.id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Plan not found".to_string()))?;

    // Get the next column order
    let max_order = sqlx::query!("SELECT MAX(column_order) as max_order FROM columns WHERE plan_id = ?", payload.plan_id)
        .fetch_one(&pool)
        .await?;

    let next_order = max_order.max_order.unwrap_or(0) + 1;

    // Create the new column
    let result = sqlx::query!(
        "INSERT INTO columns (plan_id, column_order, column_type, interruption_time) VALUES (?, ?, 'revision', ?)",
        payload.plan_id,
        next_order,
        payload.interruption_time
    )
    .execute(&pool)
    .await?;

    let column_id = result.last_insert_rowid();

    let column = sqlx::query_as!(
        Column,
        "SELECT * FROM columns WHERE id = ?",
        column_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(column))
}

pub async fn delete_column(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    // Verify column belongs to user's plan and is not the original column
    let column = sqlx::query!(
        "SELECT c.id, c.column_type FROM columns c
         INNER JOIN plans p ON c.plan_id = p.id
         WHERE c.id = ? AND p.user_id = ?",
        id,
        auth_user.0.id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Column not found".to_string()))?;

    if column.column_type == "original" {
        return Err(AppError::BadRequest("Cannot delete the original column".to_string()));
    }

    sqlx::query!("DELETE FROM columns WHERE id = ?", id)
        .execute(&pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
