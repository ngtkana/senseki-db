mod match_form;
mod match_item;
mod match_list;
mod session_header;

use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{Character, Match, Session};

pub use match_list::MatchList;
pub use session_header::SessionHeader;

#[component]
pub fn MainContent(
    sessions: ReadSignal<Vec<Session>>,
    characters: ReadSignal<Vec<Character>>,
    selected_session_id: ReadSignal<Option<i32>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_sessions_reload: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (matches, set_matches) = signal(Vec::<Match>::new());
    let (loading_matches, set_loading_matches) = signal(false);
    let (session_version, set_session_version) = signal(0);

    // セッション選択時にマッチを取得
    Effect::new(move || {
        if let Some(session_id) = selected_session_id.get() {
            set_loading_matches.set(true);
            spawn_local(async move {
                match crate::api::fetch_matches(session_id).await {
                    Ok(data) => set_matches.set(data),
                    Err(e) => logging::error!("マッチ取得失敗: {}", e),
                }
                set_loading_matches.set(false);
            });
        }
    });

    let reload_matches = move || {
        if let Some(session_id) = selected_session_id.get() {
            spawn_local(async move {
                match crate::api::fetch_matches(session_id).await {
                    Ok(data) => set_matches.set(data),
                    Err(e) => logging::error!("マッチ取得失敗: {}", e),
                }
            });
        }
    };

    let reload_session = move || {
        set_session_version.update(|v| *v += 1);
        on_sessions_reload();
    };

    view! {
        <div class="main-content">
            {move || {
                match selected_session_id.get() {
                    None => {
                        view! {
                            <div class="empty-state">"左側からセッションを選択してください"</div>
                        }
                            .into_any()
                    }
                    Some(session_id) => {
                        let session = sessions
                            .get()
                            .iter()
                            .find(|s| s.id == session_id)
                            .cloned();
                        match session {
                            Some(s) => {
                                view! {
                                    <div>
                                        <SessionHeader
                                            session=s
                                            session_id=session_id
                                            on_session_updated=reload_session
                                            _version=session_version
                                        />

                                        {move || {
                                            if loading_matches.get() {
                                                view! { <div class="loading">"読み込み中..."</div> }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <MatchList
                                                        session_id=session_id
                                                        matches=matches
                                                        characters=characters
                                                        selected_character_id=selected_character_id
                                                        on_match_added=reload_matches
                                                    />
                                                }
                                                    .into_any()
                                            }
                                        }}

                                    </div>
                                }
                                    .into_any()
                            }
                            None => {
                                view! { <div class="empty-state">"セッションが見つかりません"</div> }
                                    .into_any()
                            }
                        }
                    }
                }
            }}

        </div>
    }
}
