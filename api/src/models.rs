use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// セッション作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub session_date: NaiveDate,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub start_gsp: Option<i32>,
    pub end_gsp: Option<i32>,
}

// セッション更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateSessionRequest {
    pub session_date: Option<NaiveDate>,
    pub title: Option<Option<String>>,
    pub notes: Option<Option<String>>,
    pub start_gsp: Option<Option<i32>>,
    pub end_gsp: Option<Option<i32>>,
}

// セッションレスポンス
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: i32,
    pub session_date: NaiveDate,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub match_count: i64,
    pub wins: i64,
    pub losses: i64,
    pub start_gsp: Option<i32>,
    pub end_gsp: Option<i32>,
}

// マッチ作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateMatchRequest {
    pub session_id: i32,
    pub character_id: i32,
    pub opponent_character_id: i32,
    pub result: String, // "win" or "loss"
    pub comment: Option<String>,
}

// マッチ更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateMatchRequest {
    pub character_id: Option<i32>,
    pub opponent_character_id: Option<i32>,
    pub result: Option<String>,
    pub comment: Option<String>,
}

// マッチレスポンス
#[derive(Debug, Serialize)]
pub struct MatchResponse {
    pub id: i32,
    pub session_id: i32,
    pub character_name: String,
    pub opponent_character_name: String,
    pub result: String,
    pub match_order: i32,
    pub comment: Option<String>,
}

// GSP記録作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateGspRecordRequest {
    pub session_id: i32,
    pub match_order: i32,
    pub gsp: i32,
    pub note: Option<String>,
}

// GSP記録レスポンス
#[derive(Debug, Serialize)]
pub struct GspRecordResponse {
    pub id: i32,
    pub session_id: i32,
    pub match_order: i32,
    pub gsp: i32,
    pub note: Option<String>,
}

// キャラクターレスポンス
#[derive(Debug, Serialize)]
pub struct CharacterResponse {
    pub id: i32,
    pub name: String,
    pub name_en: String,
    pub fighter_key: String,
}
