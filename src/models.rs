use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

// User models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
}

// Session model
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

// Plan models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Plan {
    pub id: i64,
    pub user_id: i64,
    pub plan_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub plan_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}

#[derive(Debug, Serialize)]
pub struct PlanResponse {
    pub plan: Plan,
    pub columns: Vec<ColumnWithBlocks>,
}

// Column models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Column {
    pub id: i64,
    pub plan_id: i64,
    pub column_order: i64,
    pub column_type: String,
    pub interruption_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct ColumnWithBlocks {
    #[serde(flatten)]
    pub column: Column,
    pub blocks: Vec<TimeBlock>,
}

#[derive(Debug, Deserialize)]
pub struct CreateColumnRequest {
    pub plan_id: i64,
    pub interruption_time: NaiveDateTime,
}

// Time block models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TimeBlock {
    pub id: i64,
    pub column_id: i64,
    pub title: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub color: Option<String>,
    pub is_completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateBlockRequest {
    pub column_id: i64,
    pub title: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBlockRequest {
    pub title: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub color: Option<String>,
    pub is_completed: Option<bool>,
}
