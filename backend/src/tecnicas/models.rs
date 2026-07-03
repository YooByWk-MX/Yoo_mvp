use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Tool {
    pub id: i32,
    pub name: String,
    pub current_qty: i32,
}

#[derive(Deserialize)]
pub struct CreateToolReq {
    pub name: String,
}

#[derive(Deserialize)]
pub struct InventoryLogReq {
    pub tool_id: i32,
    pub log_type: String, // "IN" or "OUT"
    pub qty: i32,
    pub supplier: Option<String>,
    pub unit_price: Option<BigDecimal>,
    pub purchase_date: Option<NaiveDate>,
    pub log_date: NaiveDate,
}

#[derive(Serialize, FromRow)]
pub struct Rental {
    pub id: i32,
    pub tool_id: i32,
    pub tool_name: String,
    pub borrower_empno: Option<String>,
    pub borrower_name: String,
    pub borrower_team: Option<String>,
    pub rent_start: DateTime<Utc>,
    pub rent_end: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateRentalReq {
    pub tool_id: i32,
    pub borrower_empno: Option<String>,
    pub borrower_name: String,
    pub borrower_team: Option<String>,
}
