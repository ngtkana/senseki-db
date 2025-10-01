use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// セッション作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub session_date: NaiveDate,
    pub notes: Option<String>,
}

// セッションレスポンス
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: i32,
    pub session_date: NaiveDate,
    pub notes: Option<String>,
    pub match_count: i64,
    pub wins: i64,
    pub losses: i64,
}

// マッチ作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateMatchRequest {
    pub session_id: i32,
    pub character_id: i32,
    pub opponent_character_id: i32,
    pub result: String, // "win" or "loss"
    pub gsp_before: Option<i32>,
    pub gsp_after: Option<i32>,
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
    pub gsp_before: Option<i32>,
    pub gsp_after: Option<i32>,
    pub comment: Option<String>,
}

// キャラクターレスポンス
#[derive(Debug, Serialize)]
pub struct CharacterResponse {
    pub id: i32,
    pub name: String,
}
