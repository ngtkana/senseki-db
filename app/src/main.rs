use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;

mod api;
mod components;
mod utils;

use api::{Character, Session};
use components::{Header, MainContent, Sidebar};

/// セッションを取得して新しい順にソート
async fn fetch_and_sort_sessions() -> Result<Vec<Session>, String> {
    let mut sessions = api::fetch_sessions().await?;
    sessions.sort_by(|a, b| b.id.cmp(&a.id));
    Ok(sessions)
}

const SELECTED_CHARACTER_KEY: &str = "senseki_selected_character_id";

fn get_stored_character_id() -> Option<i32> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let value = storage.get_item(SELECTED_CHARACTER_KEY).ok()??;
    value.parse::<i32>().ok()
}

fn store_character_id(id: i32) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(SELECTED_CHARACTER_KEY, &id.to_string());
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (sessions, set_sessions) = signal(Vec::<Session>::new());
    let (characters, set_characters) = signal(Vec::<Character>::new());
    let (selected_session_id, set_selected_session_id) = signal(Option::<i32>::None);
    let (selected_character_id, set_selected_character_id) = signal(Option::<i32>::None);
    let (loading, set_loading) = signal(true);

    // 初回データ取得
    spawn_local(async move {
        if let Ok(data) = fetch_and_sort_sessions().await {
            // 最新セッションを自動選択
            if let Some(latest) = data.first() {
                set_selected_session_id.set(Some(latest.id));
            }
            set_sessions.set(data);
        }
        if let Ok(data) = api::fetch_characters().await {
            // localStorageから保存されたキャラクターIDを読み込む
            let stored_id = get_stored_character_id();
            if let Some(id) = stored_id {
                // 保存されたIDが有効か確認
                if data.iter().any(|c| c.id == id) {
                    set_selected_character_id.set(Some(id));
                } else if let Some(first_char) = data.first() {
                    set_selected_character_id.set(Some(first_char.id));
                }
            } else if let Some(first_char) = data.first() {
                // 保存されていない場合は最初のキャラクター（マリオ）を自動選択
                set_selected_character_id.set(Some(first_char.id));
            }
            set_characters.set(data);
        }
        set_loading.set(false);
    });

    let reload_sessions = move || {
        let current_selected = selected_session_id.get_untracked();
        spawn_local(async move {
            if let Ok(data) = fetch_and_sort_sessions().await {
                // 現在選択中のセッションが削除された場合、別のセッションを選択
                if let Some(current_id) = current_selected {
                    if !data.iter().any(|s| s.id == current_id) {
                        // 削除されたので、最新のセッションを選択
                        if let Some(latest) = data.first() {
                            set_selected_session_id.set(Some(latest.id));
                        } else {
                            set_selected_session_id.set(None);
                        }
                    }
                }

                set_sessions.set(data);
            }
        });
    };

    let handle_session_added = move |new_session_id: i32| {
        spawn_local(async move {
            if let Ok(data) = fetch_and_sort_sessions().await {
                set_sessions.set(data);
                // 新しく追加されたセッションを選択
                set_selected_session_id.set(Some(new_session_id));
            }
        });
    };

    let handle_character_select = move |id: i32| {
        set_selected_character_id.set(Some(id));
        store_character_id(id);
    };

    let handle_session_prev = move || {
        let current_id = selected_session_id.get();
        let all_sessions = sessions.get();
        if let Some(current) = current_id {
            if let Some(current_index) = all_sessions.iter().position(|s| s.id == current) {
                if current_index + 1 < all_sessions.len() {
                    set_selected_session_id.set(Some(all_sessions[current_index + 1].id));
                }
            }
        }
    };

    let handle_session_next = move || {
        let current_id = selected_session_id.get();
        let all_sessions = sessions.get();
        if let Some(current) = current_id {
            if let Some(current_index) = all_sessions.iter().position(|s| s.id == current) {
                if current_index > 0 {
                    set_selected_session_id.set(Some(all_sessions[current_index - 1].id));
                }
            }
        }
    };

    let handle_session_add = move || {
        spawn_local(async move {
            let req = api::CreateSessionRequest {
                session_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
                title: None,
                notes: None,
                start_gsp: None,
                end_gsp: None,
            };
            match api::create_session(req).await {
                Ok(new_session) => {
                    if let Ok(data) = fetch_and_sort_sessions().await {
                        set_sessions.set(data);
                        set_selected_session_id.set(Some(new_session.id));
                    }
                }
                Err(e) => {
                    leptos::logging::error!("セッション追加失敗: {}", e);
                }
            }
        });
    };

    let handle_session_delete = move || {
        let current_id = selected_session_id.get_untracked();
        if let Some(session_id) = current_id {
            spawn_local(async move {
                match api::delete_session(session_id).await {
                    Ok(_) => {
                        leptos::logging::log!("セッション削除成功");
                        if let Ok(data) = fetch_and_sort_sessions().await {
                            // 削除後、最新のセッションを選択
                            if let Some(latest) = data.first() {
                                set_selected_session_id.set(Some(latest.id));
                            } else {
                                set_selected_session_id.set(None);
                            }
                            set_sessions.set(data);
                        }
                    }
                    Err(e) => {
                        leptos::logging::error!("セッション削除失敗: {}", e);
                    }
                }
            });
        }
    };

    view! {
        <div class="app">
            <Header
                characters=characters
                selected_character_id=selected_character_id
                on_character_select=handle_character_select
                sessions=sessions
                selected_session_id=selected_session_id
                on_session_prev=handle_session_prev
                on_session_next=handle_session_next
                on_session_add=handle_session_add
                on_session_delete=handle_session_delete
            />

            <div class="app-main">
                <Sidebar
                    sessions=sessions
                    selected_session_id=selected_session_id
                    on_select=move |id| set_selected_session_id.set(Some(id))
                    on_session_deleted=reload_sessions
                    on_session_added=handle_session_added
                    loading=loading
                />

                <MainContent
                    sessions=sessions
                    characters=characters
                    selected_session_id=selected_session_id
                    selected_character_id=selected_character_id
                    on_sessions_reload=reload_sessions
                />
            </div>

        </div>
    }
}
