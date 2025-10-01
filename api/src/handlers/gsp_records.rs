use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::{gsp_records, prelude::*};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};

use crate::{
    models::{CreateGspRecordRequest, GspRecordResponse},
    AppState,
};

// GSP記録作成
pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateGspRecordRequest>,
) -> impl IntoResponse {
    let new_record = gsp_records::ActiveModel {
        session_id: Set(req.session_id),
        match_order: Set(req.match_order),
        gsp: Set(req.gsp),
        note: Set(req.note),
        ..Default::default()
    };

    match new_record.insert(&state.db).await {
        Ok(record) => {
            let response = GspRecordResponse {
                id: record.id,
                session_id: record.session_id,
                match_order: record.match_order,
                gsp: record.gsp,
                note: record.note,
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create GSP record: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create GSP record"
                })),
            )
                .into_response()
        }
    }
}

// セッションのGSP記録一覧取得
pub async fn list_by_session(
    State(state): State<AppState>,
    Path(session_id): Path<i32>,
) -> impl IntoResponse {
    let records = GspRecords::find()
        .filter(gsp_records::Column::SessionId.eq(session_id))
        .order_by_asc(gsp_records::Column::MatchOrder)
        .all(&state.db)
        .await;

    match records {
        Ok(records) => {
            let response: Vec<GspRecordResponse> = records
                .into_iter()
                .map(|r| GspRecordResponse {
                    id: r.id,
                    session_id: r.session_id,
                    match_order: r.match_order,
                    gsp: r.gsp,
                    note: r.note,
                })
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch GSP records: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch GSP records"
                })),
            )
                .into_response()
        }
    }
}
