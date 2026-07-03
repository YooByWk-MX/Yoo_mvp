use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::CorsLayer;

mod attendance;
mod error;
mod jig;
mod meals;
mod middleware;
mod tecnicas;
mod users;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .expect("DB 연결 실패");
    /**
       sqlx::migrate!("./migrations")
           .run(&db_pool)
           .await
           .expect("DB 마이그레이션 실패");
    */
    let state = AppState {
        db_pool,
        jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()), // <- 개발 이후 수정
    };

    // sqlx::query(
    //     "CREATE TABLE IF NOT EXISTS jigs (
    //         no BIGSERIAL PRIMARY KEY, prov TEXT NOT NULL, cantidad INTEGER NOT NULL,
    //         tablero TEXT, color TEXT, mf TEXT, types TEXT, pin TEXT, note TEXT,
    //         created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    //         updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    //         deleted TIMESTAMP WITH TIME ZONE
    //     );",
    // )
    // .execute(&pool)
    // .await
    // .expect("Failed to create table");

    // let jig_repo = jig::repository::JigRepository::new(pool);
    // let state = AppState { jig_repo };
    let cors = CorsLayer::permissive();

    /*

        let app = Router::new()
            // 헬스체크
            .route("/health", get(|| async { "Server is healthy" }))
            // 기타 라우팅 : 지그 외 (추가 예정)
            // 유저 라우팅
            .nest("/api/users", users::public_router())
            // 인증 영역
            .nest("/api/attendance", attendance::router())
            .nest("/api/meals", meals::router())
            // .nest("/api/jigs", jig::router())
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                middleware::auth::require_login,
            ))
            // 상태 설정
            .with_state(state)
            .layer(cors);
    */

    // 토큰 불필요
    let public_routes = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/api/users", users::public_router());

    // 토큰 필요
    let private_routes = Router::new()
        .nest("/api/attendance", attendance::router())
        .nest("/api/meals", meals::router())
        .nest(
            "/api/tecnicas",
            tecnicas::router().layer(axum::middleware::from_fn_with_state(
                state.clone(),
                middleware::auth::require_admin,
            )),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::require_login,
        ));

    let private_routes = Router::new()
        .nest("/api/attendance", attendance::router())
        .nest("/api/meals", meals::router())
        // 생기팀 전용 모듈 (미들웨어로 ADMIN 권한 강제)
        .nest(
            "/api/tecnicas",
            tecnicas::router().layer(axum::middleware::from_fn_with_state(
                state.clone(),
                middleware::auth::require_admin,
            )),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::require_login,
        ));

    let app = Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Server running on all interfaces at http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
