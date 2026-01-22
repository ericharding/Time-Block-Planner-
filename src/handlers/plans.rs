use crate::{
    auth::AuthUser,
    handlers::auth::AppError,
    models::{Column, ColumnWithBlocks, CreatePlanRequest, Plan, PlanResponse, TimeBlock, UpdatePlanRequest},
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{NaiveDate, Utc};
use sqlx::SqlitePool;

pub async fn list_plans(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Plan>>, AppError> {
    let plans = sqlx::query_as!(
        Plan,
        "SELECT * FROM plans WHERE user_id = ? ORDER BY plan_date DESC",
        auth_user.0.id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(plans))
}

pub async fn get_plan(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Path(date): Path<NaiveDate>,
) -> Result<Json<PlanResponse>, AppError> {
    // Fetch or create plan for the date
    let plan = sqlx::query_as!(
        Plan,
        "SELECT * FROM plans WHERE user_id = ? AND plan_date = ?",
        auth_user.0.id,
        date
    )
    .fetch_optional(&pool)
    .await?;

    let plan = match plan {
        Some(p) => p,
        None => {
            // Create a default plan
            let result = sqlx::query!(
                "INSERT INTO plans (user_id, plan_date, start_time, end_time) VALUES (?, ?, '08:00:00', '17:00:00')",
                auth_user.0.id,
                date
            )
            .execute(&pool)
            .await?;

            let plan_id = result.last_insert_rowid();

            // Create the original column
            sqlx::query!(
                "INSERT INTO columns (plan_id, column_order, column_type) VALUES (?, 0, 'original')",
                plan_id
            )
            .execute(&pool)
            .await?;

            sqlx::query_as!(
                Plan,
                "SELECT * FROM plans WHERE id = ?",
                plan_id
            )
            .fetch_one(&pool)
            .await?
        }
    };

    // Fetch columns with their blocks
    let columns = fetch_columns_with_blocks(&pool, plan.id).await?;

    Ok(Json(PlanResponse { plan, columns }))
}

pub async fn create_plan(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreatePlanRequest>,
) -> Result<Json<PlanResponse>, AppError> {
    // Check if plan already exists
    let existing = sqlx::query!("SELECT id FROM plans WHERE user_id = ? AND plan_date = ?", auth_user.0.id, payload.plan_date)
        .fetch_optional(&pool)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Plan for this date already exists".to_string()));
    }

    let result = sqlx::query!(
        "INSERT INTO plans (user_id, plan_date, start_time, end_time) VALUES (?, ?, ?, ?)",
        auth_user.0.id,
        payload.plan_date,
        payload.start_time,
        payload.end_time
    )
    .execute(&pool)
    .await?;

    let plan_id = result.last_insert_rowid();

    // Create the original column
    sqlx::query!(
        "INSERT INTO columns (plan_id, column_order, column_type) VALUES (?, 0, 'original')",
        plan_id
    )
    .execute(&pool)
    .await?;

    let plan = sqlx::query_as!(Plan, "SELECT * FROM plans WHERE id = ?", plan_id)
        .fetch_one(&pool)
        .await?;

    let columns = fetch_columns_with_blocks(&pool, plan.id).await?;

    Ok(Json(PlanResponse { plan, columns }))
}

pub async fn update_plan(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePlanRequest>,
) -> Result<Json<PlanResponse>, AppError> {
    // Verify ownership
    let plan = sqlx::query_as!(Plan, "SELECT * FROM plans WHERE id = ? AND user_id = ?", id, auth_user.0.id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Plan not found".to_string()))?;

    let now = Utc::now().naive_utc();

    if let Some(start_time) = payload.start_time {
        sqlx::query!("UPDATE plans SET start_time = ?, updated_at = ? WHERE id = ?", start_time, now, id)
            .execute(&pool)
            .await?;
    }

    if let Some(end_time) = payload.end_time {
        sqlx::query!("UPDATE plans SET end_time = ?, updated_at = ? WHERE id = ?", end_time, now, id)
            .execute(&pool)
            .await?;
    }

    let plan = sqlx::query_as!(Plan, "SELECT * FROM plans WHERE id = ?", id)
        .fetch_one(&pool)
        .await?;

    let columns = fetch_columns_with_blocks(&pool, plan.id).await?;

    Ok(Json(PlanResponse { plan, columns }))
}

pub async fn delete_plan(
    auth_user: AuthUser,
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!("DELETE FROM plans WHERE id = ? AND user_id = ?", id, auth_user.0.id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Plan not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Helper function to fetch columns with their blocks
async fn fetch_columns_with_blocks(
    pool: &SqlitePool,
    plan_id: i64,
) -> Result<Vec<ColumnWithBlocks>, sqlx::Error> {
    let columns = sqlx::query_as!(
        Column,
        "SELECT * FROM columns WHERE plan_id = ? ORDER BY column_order",
        plan_id
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();

    for column in columns {
        let blocks = sqlx::query_as!(
            TimeBlock,
            "SELECT * FROM time_blocks WHERE column_id = ? ORDER BY start_time",
            column.id
        )
        .fetch_all(pool)
        .await?;

        result.push(ColumnWithBlocks { column, blocks });
    }

    Ok(result)
}

use axum::http::StatusCode;
