use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{matches, prelude::*};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};

use crate::{
    models::{CreateMatchRequest, MatchResponse},
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
        gsp_before: Set(req.gsp_before),
        gsp_after: Set(req.gsp_after),
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
                gsp_before: match_record.gsp_before,
                gsp_after: match_record.gsp_after,
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
                    gsp_before: match_record.gsp_before,
                    gsp_after: match_record.gsp_after,
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
