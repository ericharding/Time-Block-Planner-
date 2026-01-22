use crate::{
    auth::AuthUser,
    handlers::auth::AppError,
    models::{CreateBlockRequest, TimeBlock, UpdateBlockRequest},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sqlx::SqlitePool;

pub async fn create_block(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateBlockRequest>,
) -> Result<Json<TimeBlock>, AppError> {
    // Verify column belongs to user's plan
    let column = sqlx::query!(
        "SELECT c.id FROM columns c
         INNER JOIN plans p ON c.plan_id = p.id
         WHERE c.id = ? AND p.user_id = ?",
        payload.column_id,
        auth_user.0.id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Column not found".to_string()))?;

    // Create the block
    let result = sqlx::query!(
        "INSERT INTO time_blocks (column_id, title, start_time, end_time, color) VALUES (?, ?, ?, ?, ?)",
        payload.column_id,
        payload.title,
        payload.start_time,
        payload.end_time,
        payload.color
    )
    .execute(&pool)
    .await?;

    let block_id = result.last_insert_rowid();

    let block = sqlx::query_as!(
        TimeBlock,
        "SELECT * FROM time_blocks WHERE id = ?",
        block_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(block))
}

pub async fn update_block(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateBlockRequest>,
) -> Result<Json<TimeBlock>, AppError> {
    // Verify block belongs to user's plan
    let block = sqlx::query!(
        "SELECT tb.id FROM time_blocks tb
         INNER JOIN columns c ON tb.column_id = c.id
         INNER JOIN plans p ON c.plan_id = p.id
         WHERE tb.id = ? AND p.user_id = ?",
        id,
        auth_user.0.id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Block not found".to_string()))?;

    let now = Utc::now().naive_utc();

    // Update fields if provided
    if let Some(title) = payload.title {
        sqlx::query!("UPDATE time_blocks SET title = ?, updated_at = ? WHERE id = ?", title, now, id)
            .execute(&pool)
            .await?;
    }

    if let Some(start_time) = payload.start_time {
        sqlx::query!("UPDATE time_blocks SET start_time = ?, updated_at = ? WHERE id = ?", start_time, now, id)
            .execute(&pool)
            .await?;
    }

    if let Some(end_time) = payload.end_time {
        sqlx::query!("UPDATE time_blocks SET end_time = ?, updated_at = ? WHERE id = ?", end_time, now, id)
            .execute(&pool)
            .await?;
    }

    if let Some(color) = payload.color {
        sqlx::query!("UPDATE time_blocks SET color = ?, updated_at = ? WHERE id = ?", color, now, id)
            .execute(&pool)
            .await?;
    }

    if let Some(is_completed) = payload.is_completed {
        sqlx::query!("UPDATE time_blocks SET is_completed = ?, updated_at = ? WHERE id = ?", is_completed, now, id)
            .execute(&pool)
            .await?;
    }

    let block = sqlx::query_as!(
        TimeBlock,
        "SELECT * FROM time_blocks WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(block))
}

pub async fn delete_block(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    // Verify block belongs to user's plan
    let result = sqlx::query!(
        "DELETE FROM time_blocks WHERE id IN (
            SELECT tb.id FROM time_blocks tb
            INNER JOIN columns c ON tb.column_id = c.id
            INNER JOIN plans p ON c.plan_id = p.id
            WHERE tb.id = ? AND p.user_id = ?
        )",
        id,
        auth_user.0.id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Block not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
