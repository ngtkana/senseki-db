use axum::{
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;

use handlers::{characters, gsp_records, matches, sessions};

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ログ初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "senseki_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 環境変数読み込み
    dotenvy::dotenv().ok();

    // データベース接続
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/senseki".to_string());

    tracing::info!("Connecting to database: {}", database_url);
    let db = Database::connect(&database_url).await?;

    let state = AppState { db };

    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ルーター構築
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        // キャラクター
        .route("/api/characters", get(characters::list))
        .route("/api/characters/{id}", get(characters::get))
        // セッション
        .route("/api/sessions", get(sessions::list).post(sessions::create))
        .route(
            "/api/sessions/{id}",
            get(sessions::get)
                .put(sessions::update)
                .delete(sessions::delete),
        )
        // マッチ
        .route("/api/matches", post(matches::create))
        .route(
            "/api/matches/{id}",
            put(matches::update).delete(matches::delete),
        )
        .route(
            "/api/sessions/{session_id}/matches",
            get(matches::list_by_session),
        )
        // GSP記録
        .route("/api/gsp_records", post(gsp_records::create))
        .route(
            "/api/sessions/{session_id}/gsp_records",
            get(gsp_records::list_by_session),
        )
        .layer(cors)
        .with_state(state);

    // サーバー起動
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "スマブラSP 戦績管理 API"
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok"
    }))
}
