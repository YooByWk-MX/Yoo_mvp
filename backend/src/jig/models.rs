use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Jig {
    pub no: i64,
    pub prov: String,
    pub cantidad: i32,
    pub tablero: Option<String>,
    pub color: Option<String>,
    pub mf: Option<String>,
    pub types: Option<String>,
    pub pin: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJigDto {
    pub prov: String,
    pub cantidad: i32,
    pub tablero: Option<String>,
    pub color: Option<String>,
    pub mf: Option<String>,
    pub types: Option<String>,
    pub pin: Option<String>,
    pub note: Option<String>,
}
