use crate::models::{Session, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use sqlx::SqlitePool;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub async fn create_session(pool: &SqlitePool, user_id: i64) -> Result<String, sqlx::Error> {
    let session_id = format!("{}", uuid::Uuid::new_v4());
    let expires_at = Utc::now().naive_utc() + Duration::days(7);

    sqlx::query!(
        "INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)",
        session_id,
        user_id,
        expires_at
    )
    .execute(pool)
    .await?;

    Ok(session_id)
}

pub async fn get_user_from_session(
    pool: &SqlitePool,
    session_id: &str,
) -> Result<Option<User>, sqlx::Error> {
    let now = Utc::now().naive_utc();

    let session = sqlx::query_as!(
        Session,
        "SELECT * FROM sessions WHERE id = ? AND expires_at > ?",
        session_id,
        now
    )
    .fetch_optional(pool)
    .await?;

    if let Some(session) = session {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", session.user_id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    } else {
        Ok(None)
    }
}

pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE id = ?", session_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Extractor for authenticated requests
pub struct AuthUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    SqlitePool: FromRequestParts<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = SqlitePool::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database connection failed"))?;

        let cookies = parts
            .extract::<CookieJar>()
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Failed to parse cookies"))?;

        let session_cookie = cookies
            .get("session_id")
            .ok_or((StatusCode::UNAUTHORIZED, "Not authenticated"))?;

        let user = get_user_from_session(&pool, session_cookie.value())
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid session"))?;

        Ok(AuthUser(user))
    }
}
