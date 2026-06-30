use crate::{AppState, error::AppError, middleware::auth::Claims};
use axum::{Json, Router, extract::State, routing::post};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginReq {
    pub empno: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginRes {
    pub token: String,
    pub role: String,
}

pub fn public_router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> Result<Json<LoginRes>, AppError> {
    // 1. DB에서 유저 조회
    let user = sqlx::query!(
        "SELECT password_hash, role FROM users WHERE empno = $1 AND deleted_at IS NULL",
        payload.empno
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("유저를 찾을 수 없습니다.".into()))?;

    // 2. 비밀번호 비교 (bcrypt)
    let valid = bcrypt::verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError("비밀번호 검증 실패".into()))?;

    if !valid {
        return Err(AppError::Unauthorized("비밀번호가 다릅니다.".into()));
    }

    // 3. JWT 발급
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::hours(48))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        empno: payload.empno.clone(),
        role: user.role.clone(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|_| AppError::InternalServerError("토큰 발급 실패".into()))?;

    Ok(Json(LoginRes {
        token,
        role: user.role,
    }))
}
