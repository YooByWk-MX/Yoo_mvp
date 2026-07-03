use axum::{extract::{State, Path, Extension}, Json};
use crate::{AppState, error::AppError, middleware::auth::Claims};
use super::models::*;

// 1. 공구 목록 조회 (NULL 값을 0으로 안전하게 매핑)
pub async fn get_tools(State(state): State<AppState>) -> Result<Json<Vec<Tool>>, AppError> {
    let tools = sqlx::query_as!(
        Tool, 
        r#"SELECT id, name, COALESCE(current_qty, 0) as "current_qty!" FROM tools ORDER BY id DESC"#
    )
    .fetch_all(&state.db_pool).await?;
    
    Ok(Json(tools))
}

// 2. 신규 공구 등록
pub async fn create_tool(State(state): State<AppState>, Json(payload): Json<CreateToolReq>) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query!("INSERT INTO tools (name) VALUES ($1)", payload.name)
        .execute(&state.db_pool).await?;
        
    Ok(Json(serde_json::json!({"message": "공구 등록 완료"})))
}

// 3. 입출고 로그 기록 및 재고 업데이트 (안전성 강화)
pub async fn add_inventory_log(
    State(state): State<AppState>, 
    Extension(claims): Extension<Claims>, 
    Json(payload): Json<InventoryLogReq>
) -> Result<Json<serde_json::Value>, AppError> {
    let mut tx = state.db_pool.begin().await?;
    
    // 현재고 조회 (NULL일 경우 0으로 처리)
    let current_qty = sqlx::query!("SELECT current_qty FROM tools WHERE id = $1", payload.tool_id)
        .fetch_one(&mut *tx).await?.current_qty.unwrap_or(0);
        
    // 증감 계산
    let new_balance = if payload.log_type == "IN" { 
        current_qty + payload.qty 
    } else { 
        current_qty - payload.qty 
    };
    
    if new_balance < 0 {
        return Err(AppError::BadRequest("재고가 부족하여 출고할 수 없습니다.".into()));
    }
    
    // 재고 업데이트
    sqlx::query!("UPDATE tools SET current_qty = $1 WHERE id = $2", new_balance, payload.tool_id)
        .execute(&mut *tx).await?;
        
    // 로그 기록
    sqlx::query!(
        r#"INSERT INTO tool_inventory_logs 
          (tool_id, log_type, qty, balance, supplier, unit_price, purchase_date, log_date, manager_empno) 
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        payload.tool_id, payload.log_type, payload.qty, new_balance, payload.supplier, 
        payload.unit_price, payload.purchase_date, payload.log_date, claims.empno
    )
    .execute(&mut *tx).await?;
        
    tx.commit().await?;
    Ok(Json(serde_json::json!({"message": "입출고 기록 완료"})))
}

// 4. 대여 목록 조회
pub async fn get_rentals(State(state): State<AppState>) -> Result<Json<Vec<Rental>>, AppError> {
    let rentals = sqlx::query_as!(
        Rental, 
        r#"SELECT r.id, r.tool_id, t.name as tool_name, r.borrower_empno, r.borrower_name, 
           r.borrower_team, r.rent_start, r.rent_end, r.status 
           FROM tool_rentals r 
           JOIN tools t ON r.tool_id = t.id 
           ORDER BY r.status ASC, r.rent_start DESC"#
    )
    .fetch_all(&state.db_pool).await?;
    
    Ok(Json(rentals))
}

// 5. 공구 대여 처리
pub async fn create_rental(
    State(state): State<AppState>, 
    Extension(claims): Extension<Claims>, 
    Json(payload): Json<CreateRentalReq>
) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query!(
        r#"INSERT INTO tool_rentals (tool_id, borrower_empno, borrower_name, borrower_team, manager_empno) 
           VALUES ($1, $2, $3, $4, $5)"#,
        payload.tool_id, payload.borrower_empno, payload.borrower_name, payload.borrower_team, claims.empno
    )
    .execute(&state.db_pool).await?;
    
    Ok(Json(serde_json::json!({"message": "대여 처리 완료"})))
}

// 6. 공구 반납 처리
pub async fn return_rental(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query!("UPDATE tool_rentals SET status = 'RETURNED', rent_end = NOW() WHERE id = $1", id)
        .execute(&state.db_pool).await?;
        
    Ok(Json(serde_json::json!({"message": "반납 완료"})))
}