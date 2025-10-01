use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;

mod api;
mod components;

use api::{Character, Session};
use components::{Header, MainContent, Sidebar};

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
        if let Ok(mut data) = api::fetch_sessions().await {
            // 新しい順にソート
            data.sort_by(|a, b| b.id.cmp(&a.id));
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
        let current_selected = selected_session_id.get();
        spawn_local(async move {
            if let Ok(mut data) = api::fetch_sessions().await {
                // 新しい順にソート
                data.sort_by(|a, b| b.id.cmp(&a.id));

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
            if let Ok(mut data) = api::fetch_sessions().await {
                // 新しい順にソート
                data.sort_by(|a, b| b.id.cmp(&a.id));
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

    view! {
        <div class="app">
            <Header
                characters=characters
                selected_character_id=selected_character_id
                on_character_select=handle_character_select
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
