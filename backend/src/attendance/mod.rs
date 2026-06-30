use crate::{AppState, error::AppError, middleware::auth::Claims};
use axum::{
    Json, Router,
    extract::{Extension, State},
    routing::{get, post},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        // 관리자 전용: 25초짜리 QR 토큰 생성
        .route("/qr-generate", get(generate_qr))
        // 작업자 전용: QR 스캔 및 출석 처리
        .route("/scan", post(scan_qr))
}

#[derive(Serialize, Deserialize)]
struct QrClaims {
    pub jti: String,
    pub exp: usize,
}

#[derive(Deserialize)]
struct ScanReq {
    pub qr_token: String,
}

async fn generate_qr(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    let claims = QrClaims {
        jti: Uuid::new_v4().to_string(),
        exp: (Utc::now() + Duration::seconds(25)).timestamp() as usize, // 25초 수명
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .unwrap();
    Ok(Json(serde_json::json!({ "qr_data": token })))
}

async fn scan_qr(
    State(state): State<AppState>,
    Extension(user): Extension<Claims>,
    Json(payload): Json<ScanReq>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 1. QR 토큰 만료 시간 및 서명 검증
    let qr_data = decode::<QrClaims>(
        &payload.qr_token,
        &DecodingKey::from_secret(state.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        AppError::BadRequest("만료되었거나 유효하지 않은 QR입니다. 다시 스캔하세요.".into())
    })?;

    // 2. DB 기록 (동일 QR 중복 방지는 UNIQUE 제약조건이 알아서 막아줌)
    let result = sqlx::query("INSERT INTO attendance_logs (empno, qr_jti) VALUES ($1, $2)")
        .bind(&user.empno)
        .bind(&qr_data.claims.jti)
        .execute(&state.db_pool)
        .await;

    match result {
        Ok(_) => Ok(Json(
            serde_json::json!({ "message": "출석이 완료되었습니다." }),
        )),
        Err(_) => Err(AppError::BadRequest(
            "이미 처리된 QR이거나 중복 출석입니다.".into(),
        )),
    }
}
