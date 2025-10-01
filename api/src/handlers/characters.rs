use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{characters, prelude::*};
use sea_orm::{EntityTrait, QueryOrder};

use crate::{models::CharacterResponse, AppState};

// キャラクター一覧取得
pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let characters = Characters::find()
        .order_by_asc(characters::Column::Name)
        .all(&state.db)
        .await;

    match characters {
        Ok(chars) => {
            let response: Vec<CharacterResponse> = chars
                .into_iter()
                .map(|c| CharacterResponse {
                    id: c.id,
                    name: c.name,
                    name_en: c.name_en,
                })
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch characters: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch characters"
                })),
            )
                .into_response()
        }
    }
}

// キャラクター取得
pub async fn get(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let character = Characters::find_by_id(id).one(&state.db).await;

    match character {
        Ok(Some(c)) => {
            let response = CharacterResponse {
                id: c.id,
                name: c.name,
                name_en: c.name_en,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Character not found"
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch character: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch character"
                })),
            )
                .into_response()
        }
    }
}
