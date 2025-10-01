use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "http://127.0.0.1:3000/api";

// キャラクター
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub name_en: String,
}

// セッション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub session_date: String,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub match_count: i64,
    pub wins: i64,
    pub losses: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionRequest {
    pub session_date: String,
    pub title: Option<String>,
    pub notes: Option<String>,
}

// マッチ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: i32,
    pub session_id: i32,
    pub character_name: String,
    pub opponent_character_name: String,
    pub result: String,
    pub match_order: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateMatchRequest {
    pub session_id: i32,
    pub character_id: i32,
    pub opponent_character_id: i32,
    pub result: String,
    pub comment: Option<String>,
}

// GSP記録
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GspRecord {
    pub id: i32,
    pub session_id: i32,
    pub match_order: i32,
    pub gsp: i32,
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateGspRecordRequest {
    pub session_id: i32,
    pub match_order: i32,
    pub gsp: i32,
    pub note: Option<String>,
}

// API関数
pub async fn fetch_characters() -> Result<Vec<Character>, String> {
    let url = format!("{}/characters", API_BASE);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

pub async fn fetch_sessions() -> Result<Vec<Session>, String> {
    let url = format!("{}/sessions", API_BASE);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

pub async fn create_session(req: CreateSessionRequest) -> Result<Session, String> {
    let url = format!("{}/sessions", API_BASE);
    Request::post(&url)
        .json(&req)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

pub async fn fetch_matches(session_id: i32) -> Result<Vec<Match>, String> {
    let url = format!("{}/sessions/{}/matches", API_BASE, session_id);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

pub async fn create_match(req: CreateMatchRequest) -> Result<Match, String> {
    let url = format!("{}/matches", API_BASE);
    Request::post(&url)
        .json(&req)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

pub async fn fetch_gsp_records(session_id: i32) -> Result<Vec<GspRecord>, String> {
    let url = format!("{}/sessions/{}/gsp_records", API_BASE, session_id);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

pub async fn create_gsp_record(req: CreateGspRecordRequest) -> Result<GspRecord, String> {
    let url = format!("{}/gsp_records", API_BASE);
    Request::post(&url)
        .json(&req)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}
