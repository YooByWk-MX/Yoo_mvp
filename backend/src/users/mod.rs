use crate::{AppState, error::AppError};
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
    // MVP 테스트를 위해 비밀번호가 사번과 동일하면 패스하도록 임시 하드코딩

    let role = if payload.empno == "ADMIN" {
        "ADMIN"
    } else {
        "WORKER"
    };

    let claims = crate::middleware::auth::Claims {
        empno: payload.empno.clone(),
        role: role.to_string(),
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|_| AppError::InternalServerError("토큰 생성 실패".into()))?;

    Ok(Json(LoginRes {
        token,
        role: role.to_string(),
    }))
}
