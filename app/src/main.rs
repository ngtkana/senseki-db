use leptos::prelude::*;
use leptos::task::spawn_local;

mod api;
mod components;

use api::{Character, Session};
use components::{Header, MainContent, Sidebar};

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
            set_characters.set(data);
        }
        set_loading.set(false);
    });

    let reload_sessions = move || {
        spawn_local(async move {
            if let Ok(mut data) = api::fetch_sessions().await {
                // 新しい順にソート
                data.sort_by(|a, b| b.id.cmp(&a.id));
                set_sessions.set(data);
            }
        });
    };

    let handle_character_select = move |id: i32| {
        set_selected_character_id.set(Some(id));
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
