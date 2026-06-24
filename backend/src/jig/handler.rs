use super::{
    models::{CreateJigDto, Jig},
    repository::JigRepository,
};
use crate::AppState;

use axum::{Json, Router, extract::State, http::StatusCode, routing::get}; // main에서 정의할 전역 상태

// 라우터 모듈화: main.rs를 더럽히지 않고 jig 관련 라우팅만 반환
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_jigs).post(create_jig))
}

async fn list_jigs(State(state): State<AppState>) -> Result<Json<Vec<Jig>>, (StatusCode, String)> {
    let jigs = state
        .jig_repo
        .find_all()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(jigs))
}

async fn create_jig(
    State(state): State<AppState>,
    Json(payload): Json<CreateJigDto>,
) -> Result<(StatusCode, Json<Jig>), (StatusCode, String)> {
    let new_jig = state
        .jig_repo
        .create(&payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(new_jig)))
}
