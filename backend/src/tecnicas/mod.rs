pub mod handler;
pub mod models;

use crate::AppState;
use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tools", get(handler::get_tools).post(handler::create_tool))
        .route("/inventory", post(handler::add_inventory_log))
        .route(
            "/rentals",
            get(handler::get_rentals).post(handler::create_rental),
        )
        .route("/rentals/:id/return", patch(handler::return_rental))
}
