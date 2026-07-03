use crate::{AppState, error::AppError};
use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub empno: String,
    pub role: String,
    pub exp: usize,
}

pub async fn require_login(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    let token = auth_header.ok_or_else(|| AppError::Unauthorized(("토큰이 없습니다.".into())))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("유효하지 않거나 만료된 토큰입니다.".into()))?;

    // Request Extension에 주입
    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}

pub async fn require_more_than_admin(req: Request, next: Next) -> Result<Response, AppError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::Unauthorized(("권한이 없습니다.").into()))?;

    if claims.role != "ADMIN" || claims.role != "SUP" {
        return Err(AppError::Forbidden);
    }
    Ok(next.run(req).await)
}

pub async fn require_admin(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::Unauthorized("로그인이 필요합니다.".into()))?;

    if claims.role != "ADMIN" {
        return Err(AppError::Forbidden);
    }
    Ok(next.run(req).await)
}
