use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "http://127.0.0.1:3000/api";

/// API呼び出しの共通ヘルパー関数（GET）
async fn api_get<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, String> {
    Request::get(url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

/// API呼び出しの共通ヘルパー関数（POST）
async fn api_post<T: Serialize, R: for<'de> Deserialize<'de>>(
    url: &str,
    body: &T,
) -> Result<R, String> {
    Request::post(url)
        .json(body)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

/// API呼び出しの共通ヘルパー関数（PUT）
async fn api_put<T: Serialize, R: for<'de> Deserialize<'de>>(
    url: &str,
    body: &T,
) -> Result<R, String> {
    Request::put(url)
        .json(body)
        .map_err(|e| format!("JSON serialize failed: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .json()
        .await
        .map_err(|e| format!("JSON parse failed: {}", e))
}

/// API呼び出しの共通ヘルパー関数（DELETE）
async fn api_delete(url: &str) -> Result<(), String> {
    Request::delete(url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    Ok(())
}

// キャラクター
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub name_en: String,
    pub fighter_key: String,
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
    pub start_gsp: Option<i32>,
    pub end_gsp: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionRequest {
    pub session_date: String,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub start_gsp: Option<i32>,
    pub end_gsp: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSessionRequest {
    pub session_date: Option<String>,
    pub title: Option<Option<String>>,
    pub notes: Option<Option<String>>,
    pub start_gsp: Option<Option<i32>>,
    pub end_gsp: Option<Option<i32>>,
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

#[derive(Debug, Serialize)]
pub struct UpdateMatchRequest {
    pub character_id: Option<i32>,
    pub opponent_character_id: Option<i32>,
    pub result: Option<String>,
    pub comment: Option<String>,
}

// GSP記録
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GspRecord {
    pub id: i32,
    pub session_id: i32,
    pub match_order: i32,
    pub gsp: i32,
    pub note: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct CreateGspRecordRequest {
    pub session_id: i32,
    pub match_order: i32,
    pub gsp: i32,
    pub note: Option<String>,
}

// API関数
pub async fn fetch_characters() -> Result<Vec<Character>, String> {
    api_get(&format!("{}/characters", API_BASE)).await
}

pub async fn fetch_sessions() -> Result<Vec<Session>, String> {
    api_get(&format!("{}/sessions", API_BASE)).await
}

pub async fn create_session(req: CreateSessionRequest) -> Result<Session, String> {
    api_post(&format!("{}/sessions", API_BASE), &req).await
}

pub async fn update_session(session_id: i32, req: UpdateSessionRequest) -> Result<Session, String> {
    api_put(&format!("{}/sessions/{}", API_BASE, session_id), &req).await
}

pub async fn fetch_matches(session_id: i32) -> Result<Vec<Match>, String> {
    api_get(&format!("{}/sessions/{}/matches", API_BASE, session_id)).await
}

pub async fn create_match(req: CreateMatchRequest) -> Result<Match, String> {
    api_post(&format!("{}/matches", API_BASE), &req).await
}

pub async fn update_match(match_id: i32, req: UpdateMatchRequest) -> Result<Match, String> {
    api_put(&format!("{}/matches/{}", API_BASE, match_id), &req).await
}

pub async fn delete_session(session_id: i32) -> Result<(), String> {
    api_delete(&format!("{}/sessions/{}", API_BASE, session_id)).await
}

pub async fn delete_match(match_id: i32) -> Result<(), String> {
    api_delete(&format!("{}/matches/{}", API_BASE, match_id)).await
}

#[allow(dead_code)]
pub async fn fetch_gsp_records(session_id: i32) -> Result<Vec<GspRecord>, String> {
    api_get(&format!("{}/sessions/{}/gsp_records", API_BASE, session_id)).await
}

#[allow(dead_code)]
pub async fn create_gsp_record(req: CreateGspRecordRequest) -> Result<GspRecord, String> {
    api_post(&format!("{}/gsp_records", API_BASE), &req).await
}
