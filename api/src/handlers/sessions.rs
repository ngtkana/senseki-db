use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{matches, prelude::*, sessions};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};

use crate::{
    models::{CreateSessionRequest, SessionResponse},
    AppState,
};

// セッション一覧取得
pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let sessions = Sessions::find()
        .order_by_desc(sessions::Column::SessionDate)
        .all(&state.db)
        .await;

    match sessions {
        Ok(sessions) => {
            let mut response = Vec::new();

            for session in sessions {
                // セッションのマッチ数と勝敗を集計
                let matches = Matches::find()
                    .filter(matches::Column::SessionId.eq(session.id))
                    .all(&state.db)
                    .await
                    .unwrap_or_default();

                let match_count = matches.len() as i64;
                let wins = matches.iter().filter(|m| m.result == "win").count() as i64;
                let losses = matches.iter().filter(|m| m.result == "loss").count() as i64;

                response.push(SessionResponse {
                    id: session.id,
                    session_date: session.session_date,
                    notes: session.notes,
                    match_count,
                    wins,
                    losses,
                });
            }

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch sessions: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch sessions"
                })),
            )
                .into_response()
        }
    }
}

// セッション取得
pub async fn get(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let session = Sessions::find_by_id(id).one(&state.db).await;

    match session {
        Ok(Some(session)) => {
            // マッチ数と勝敗を集計
            let matches = Matches::find()
                .filter(matches::Column::SessionId.eq(session.id))
                .all(&state.db)
                .await
                .unwrap_or_default();

            let match_count = matches.len() as i64;
            let wins = matches.iter().filter(|m| m.result == "win").count() as i64;
            let losses = matches.iter().filter(|m| m.result == "loss").count() as i64;

            let response = SessionResponse {
                id: session.id,
                session_date: session.session_date,
                notes: session.notes,
                match_count,
                wins,
                losses,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Session not found"
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch session: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch session"
                })),
            )
                .into_response()
        }
    }
}

// セッション作成
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    let new_session = sessions::ActiveModel {
        session_date: Set(req.session_date),
        notes: Set(req.notes),
        ..Default::default()
    };

    match new_session.insert(&state.db).await {
        Ok(session) => {
            let response = SessionResponse {
                id: session.id,
                session_date: session.session_date,
                notes: session.notes,
                match_count: 0,
                wins: 0,
                losses: 0,
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create session: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create session"
                })),
            )
                .into_response()
        }
    }
}
