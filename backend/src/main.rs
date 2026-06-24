use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer; // Any 삭제

mod jig;

#[derive(Clone)]
pub struct AppState {
    pub jig_repo: jig::repository::JigRepository,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS jigs (
            no BIGSERIAL PRIMARY KEY, prov TEXT NOT NULL, cantidad INTEGER NOT NULL,
            tablero TEXT, color TEXT, mf TEXT, types TEXT, pin TEXT, note TEXT,
            created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            deleted TIMESTAMP WITH TIME ZONE
        );",
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let jig_repo = jig::repository::JigRepository::new(pool);
    let state = AppState { jig_repo };

    // [수정됨] MVP 로컬 테스트를 위해 모든 CORS 제약을 무시하고 완벽 허용
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .nest("/api/jigs", jig::handler::router())
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on all interfaces at http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
