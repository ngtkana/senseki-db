use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{matches, prelude::*};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};

use crate::{
    models::{CreateMatchRequest, MatchResponse, UpdateMatchRequest},
    AppState,
};

// マッチ作成
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateMatchRequest>,
) -> impl IntoResponse {
    // セッション内の最大match_orderを取得
    let max_order = Matches::find()
        .filter(matches::Column::SessionId.eq(req.session_id))
        .order_by_desc(matches::Column::MatchOrder)
        .one(&state.db)
        .await
        .ok()
        .flatten()
        .map(|m| m.match_order)
        .unwrap_or(0);

    let new_match = matches::ActiveModel {
        session_id: Set(req.session_id),
        character_id: Set(req.character_id),
        opponent_character_id: Set(req.opponent_character_id),
        result: Set(req.result),
        match_order: Set(max_order + 1),
        comment: Set(req.comment),
        ..Default::default()
    };

    match new_match.insert(&state.db).await {
        Ok(match_record) => {
            // キャラクター名を取得
            let character = Characters::find_by_id(match_record.character_id)
                .one(&state.db)
                .await
                .ok()
                .flatten();
            let opponent = Characters::find_by_id(match_record.opponent_character_id)
                .one(&state.db)
                .await
                .ok()
                .flatten();

            let response = MatchResponse {
                id: match_record.id,
                session_id: match_record.session_id,
                character_name: character.map(|c| c.name).unwrap_or_default(),
                opponent_character_name: opponent.map(|c| c.name).unwrap_or_default(),
                result: match_record.result,
                match_order: match_record.match_order,
                comment: match_record.comment,
            };

            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create match: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create match"
                })),
            )
                .into_response()
        }
    }
}

// マッチ更新
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateMatchRequest>,
) -> impl IntoResponse {
    // 既存のマッチを取得
    let existing_match = Matches::find_by_id(id).one(&state.db).await;

    match existing_match {
        Ok(Some(match_record)) => {
            let mut active_match: matches::ActiveModel = match_record.into();

            // 更新するフィールドのみ設定
            if let Some(character_id) = req.character_id {
                active_match.character_id = Set(character_id);
            }
            if let Some(opponent_character_id) = req.opponent_character_id {
                active_match.opponent_character_id = Set(opponent_character_id);
            }
            if let Some(result) = req.result {
                active_match.result = Set(result);
            }
            if let Some(comment) = req.comment {
                active_match.comment = Set(Some(comment));
            }

            match active_match.update(&state.db).await {
                Ok(updated_match) => {
                    // キャラクター名を取得
                    let character = Characters::find_by_id(updated_match.character_id)
                        .one(&state.db)
                        .await
                        .ok()
                        .flatten();
                    let opponent = Characters::find_by_id(updated_match.opponent_character_id)
                        .one(&state.db)
                        .await
                        .ok()
                        .flatten();

                    let response = MatchResponse {
                        id: updated_match.id,
                        session_id: updated_match.session_id,
                        character_name: character.map(|c| c.name).unwrap_or_default(),
                        opponent_character_name: opponent.map(|c| c.name).unwrap_or_default(),
                        result: updated_match.result,
                        match_order: updated_match.match_order,
                        comment: updated_match.comment,
                    };

                    (StatusCode::OK, Json(response)).into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to update match: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "Failed to update match"
                        })),
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Match not found"
            })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch match: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch match"
                })),
            )
                .into_response()
        }
    }
}

// セッションのマッチ一覧取得
pub async fn list_by_session(
    State(state): State<AppState>,
    Path(session_id): Path<i32>,
) -> impl IntoResponse {
    let matches = Matches::find()
        .filter(matches::Column::SessionId.eq(session_id))
        .order_by_asc(matches::Column::MatchOrder)
        .all(&state.db)
        .await;

    match matches {
        Ok(matches) => {
            let mut response = Vec::new();

            for match_record in matches {
                // キャラクター名を取得
                let character = Characters::find_by_id(match_record.character_id)
                    .one(&state.db)
                    .await
                    .ok()
                    .flatten();
                let opponent = Characters::find_by_id(match_record.opponent_character_id)
                    .one(&state.db)
                    .await
                    .ok()
                    .flatten();

                response.push(MatchResponse {
                    id: match_record.id,
                    session_id: match_record.session_id,
                    character_name: character.map(|c| c.name).unwrap_or_default(),
                    opponent_character_name: opponent.map(|c| c.name).unwrap_or_default(),
                    result: match_record.result,
                    match_order: match_record.match_order,
                    comment: match_record.comment,
                });
            }

            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch matches: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch matches"
                })),
            )
                .into_response()
        }
    }
}
