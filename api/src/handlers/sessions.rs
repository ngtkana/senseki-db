use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{matches, prelude::*, sessions};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};

use crate::{
    models::{CreateSessionRequest, SessionResponse, UpdateSessionRequest},
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
                    title: session.title,
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
                title: session.title,
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
        title: Set(req.title),
        notes: Set(req.notes),
        ..Default::default()
    };

    match new_session.insert(&state.db).await {
        Ok(session) => {
            let response = SessionResponse {
                id: session.id,
                session_date: session.session_date,
                title: session.title,
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

// セッション更新
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateSessionRequest>,
) -> impl IntoResponse {
    // 既存のセッションを取得
    let session = Sessions::find_by_id(id).one(&state.db).await;

    match session {
        Ok(Some(session)) => {
            let mut active_session: sessions::ActiveModel = session.into();

            // 更新するフィールドを設定
            if let Some(session_date) = req.session_date {
                active_session.session_date = Set(session_date);
            }
            if let Some(title) = req.title {
                active_session.title = Set(title);
            }
            if let Some(notes) = req.notes {
                active_session.notes = Set(notes);
            }

            match active_session.update(&state.db).await {
                Ok(updated_session) => {
                    // マッチ数と勝敗を集計
                    let matches = Matches::find()
                        .filter(matches::Column::SessionId.eq(updated_session.id))
                        .all(&state.db)
                        .await
                        .unwrap_or_default();

                    let match_count = matches.len() as i64;
                    let wins = matches.iter().filter(|m| m.result == "win").count() as i64;
                    let losses = matches.iter().filter(|m| m.result == "loss").count() as i64;

                    let response = SessionResponse {
                        id: updated_session.id,
                        session_date: updated_session.session_date,
                        title: updated_session.title,
                        notes: updated_session.notes,
                        match_count,
                        wins,
                        losses,
                    };
                    (StatusCode::OK, Json(response)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to update session: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "Failed to update session"
                        })),
                    )
                        .into_response()
                }
            }
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

// セッション削除
pub async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    // セッションを取得
    let session = Sessions::find_by_id(id).one(&state.db).await;

    match session {
        Ok(Some(session)) => {
            let active_session: sessions::ActiveModel = session.into();
            match active_session.delete(&state.db).await {
                Ok(_) => (StatusCode::NO_CONTENT).into_response(),
                Err(e) => {
                    tracing::error!("Failed to delete session: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "Failed to delete session"
                        })),
                    )
                        .into_response()
                }
            }
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
