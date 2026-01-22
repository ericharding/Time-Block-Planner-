use crate::{
    auth::{self, AuthUser},
    models::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use sqlx::SqlitePool;
use time::{Duration, OffsetDateTime};

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(CookieJar, Json<AuthResponse>), AppError> {
    // Validate input
    if payload.username.is_empty() || payload.password.len() < 6 {
        return Err(AppError::BadRequest(
            "Username cannot be empty and password must be at least 6 characters".to_string(),
        ));
    }

    // Check if user already exists
    let existing_user = sqlx::query!("SELECT id FROM users WHERE username = ?", payload.username)
        .fetch_optional(&pool)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    // Hash password
    let password_hash = auth::hash_password(&payload.password)
        .map_err(|_| AppError::Internal("Failed to hash password".to_string()))?;

    // Create user
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        payload.username,
        password_hash
    )
    .execute(&pool)
    .await?;

    let user_id = result.last_insert_rowid();

    // Fetch created user
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", user_id)
        .fetch_one(&pool)
        .await?;

    // Create session
    let session_id = auth::create_session(&pool, user_id).await?;

    // Create session cookie
    let cookie = Cookie::build(("session_id", session_id.clone()))
        .path("/")
        .max_age(Duration::days(7))
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    let jar = CookieJar::new().add(cookie);

    Ok((
        jar,
        Json(AuthResponse {
            user: UserResponse {
                id: user.id,
                username: user.username,
            },
            session_id,
        }),
    ))
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(CookieJar, Json<AuthResponse>), AppError> {
    // Find user
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = ?",
        payload.username
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    let valid = auth::verify_password(&payload.password, &user.password_hash)
        .map_err(|_| AppError::Internal("Failed to verify password".to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Create session
    let session_id = auth::create_session(&pool, user.id).await?;

    // Create session cookie
    let cookie = Cookie::build(("session_id", session_id.clone()))
        .path("/")
        .max_age(Duration::days(7))
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    let jar = CookieJar::new().add(cookie);

    Ok((
        jar,
        Json(AuthResponse {
            user: UserResponse {
                id: user.id,
                username: user.username,
            },
            session_id,
        }),
    ))
}

pub async fn logout(
    State(pool): State<SqlitePool>,
    cookies: CookieJar,
) -> Result<CookieJar, AppError> {
    if let Some(cookie) = cookies.get("session_id") {
        auth::delete_session(&pool, cookie.value()).await?;
    }

    let cookie = Cookie::build(("session_id", ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .build();

    Ok(cookies.add(cookie))
}

pub async fn me(auth_user: AuthUser) -> Json<UserResponse> {
    Json(UserResponse {
        id: auth_user.0.id,
        username: auth_user.0.username,
    })
}

// Error type for handlers
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
