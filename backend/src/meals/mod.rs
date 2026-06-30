use crate::{AppState, error::AppError, middleware::auth::Claims};
use axum::{
    Json, Router,
    extract::{Extension, State},
    routing::post,
};
use chrono::NaiveDate;
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/request", post(request_meal))
}

#[derive(Deserialize)]
struct MealReq {
    pub target_date: NaiveDate,
    pub meal_type: String,
}

async fn request_meal(
    State(state): State<AppState>,
    Extension(user): Extension<Claims>,
    Json(payload): Json<MealReq>,
) -> Result<Json<serde_json::Value>, AppError> {
    let result = sqlx::query(
        "INSERT INTO meal_requests (empno, target_date, meal_type) VALUES ($1, $2, $3)",
    )
    .bind(&user.empno)
    .bind(&payload.target_date)
    .bind(&payload.meal_type)
    .execute(&state.db_pool)
    .await;

    match result {
        Ok(_) => Ok(Json(serde_json::json!({ "message": "식사 신청 완료" }))),
        Err(_) => Err(AppError::BadRequest(
            "이미 해당 끼니를 신청하셨습니다.".into(),
        )),
    }
}
