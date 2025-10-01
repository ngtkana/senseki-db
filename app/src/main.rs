use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;

mod api;

use api::{
    Character, CreateMatchRequest, CreateSessionRequest, Match, Session, UpdateMatchRequest,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (show_session_modal, set_show_session_modal) = signal(false);
    let (sessions, set_sessions) = signal(Vec::<Session>::new());
    let (characters, set_characters) = signal(Vec::<Character>::new());
    let (selected_session_id, set_selected_session_id) = signal(Option::<i32>::None);
    let (selected_character_id, set_selected_character_id) = signal(Option::<i32>::None);
    let (loading, set_loading) = signal(true);

    // 初回データ取得
    spawn_local(async move {
        if let Ok(data) = api::fetch_sessions().await {
            set_sessions.set(data);
        }
        if let Ok(data) = api::fetch_characters().await {
            set_characters.set(data);
        }
        set_loading.set(false);
    });

    let reload_sessions = move || {
        spawn_local(async move {
            if let Ok(data) = api::fetch_sessions().await {
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
                    on_new_session=move || set_show_session_modal.set(true)
                    loading=loading
                />

                <MainContent
                    sessions=sessions
                    characters=characters
                    selected_session_id=selected_session_id
                    selected_character_id=selected_character_id
                />
            </div>

            <Show when=move || show_session_modal.get()>
                <Modal on_close=move || set_show_session_modal.set(false)>
                    <SessionForm
                        on_submit=move || {
                            set_show_session_modal.set(false);
                            reload_sessions();
                        }
                    />
                </Modal>
            </Show>
        </div>
    }
}

#[component]
fn Header(
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_character_select: impl Fn(i32) + 'static + Copy,
) -> impl IntoView {
    view! {
        <header class="header">
            <h1>"スマブラSP 戦績管理"</h1>
            <div class="header-character">
                <label>"使用キャラ: "</label>
                <select
                    class="character-select"
                    on:change=move |ev| {
                        let id = event_target_value(&ev).parse().unwrap_or(0);
                        if id > 0 {
                            on_character_select(id);
                        }
                    }
                >

                    <option value="0">"選択してください"</option>
                    {move || {
                        characters
                            .get()
                            .iter()
                            .map(|c| {
                                let is_selected = selected_character_id.get() == Some(c.id);
                                view! {
                                    <option value=c.id selected=is_selected>
                                        {c.name.clone()}
                                    </option>
                                }
                            })
                            .collect_view()
                    }}

                </select>
            </div>
        </header>
    }
}

#[component]
fn Sidebar(
    sessions: ReadSignal<Vec<Session>>,
    selected_session_id: ReadSignal<Option<i32>>,
    on_select: impl Fn(i32) + 'static + Copy + Send,
    on_new_session: impl Fn() + 'static + Copy + Send,
    loading: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="sidebar">
            {move || {
                if loading.get() {
                    view! { <div class="loading">"読み込み中..."</div> }.into_any()
                } else {
                    view! {
                        <div>
                            {sessions
                                .get()
                                .iter()
                                .map(|session| {
                                    let session_id = session.id;
                                    let is_selected = move || {
                                        selected_session_id.get() == Some(session_id)
                                    };
                                    view! {
                                        <div
                                            class="session-item"
                                            class:selected=is_selected
                                            on:click=move |_| on_select(session_id)
                                        >
                                            <div class="session-date">
                                                {session.session_date.clone()}
                                                {session
                                                    .title
                                                    .as_ref()
                                                    .map(|t| format!(" {}", t))
                                                    .unwrap_or_default()}
                                            </div>
                                            <div class="session-note">
                                                {session
                                                    .notes
                                                    .clone()
                                                    .unwrap_or_else(|| "".to_string())}
                                            </div>
                                            <div class="session-stats">
                                                {format!("{}勝 {}敗", session.wins, session.losses)}
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()}

                            <button class="add-session-button" on:click=move |_| on_new_session()>
                                "+ セッション"
                            </button>
                        </div>
                    }
                        .into_any()
                }
            }}

        </div>
    }
}

#[component]
fn MainContent(
    sessions: ReadSignal<Vec<Session>>,
    characters: ReadSignal<Vec<Character>>,
    selected_session_id: ReadSignal<Option<i32>>,
    selected_character_id: ReadSignal<Option<i32>>,
) -> impl IntoView {
    let (matches, set_matches) = signal(Vec::<Match>::new());
    let (loading_matches, set_loading_matches) = signal(false);

    // セッション選択時にマッチを取得
    Effect::new(move || {
        if let Some(session_id) = selected_session_id.get() {
            set_loading_matches.set(true);
            spawn_local(async move {
                match api::fetch_matches(session_id).await {
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
                match api::fetch_matches(session_id).await {
                    Ok(data) => set_matches.set(data),
                    Err(e) => logging::error!("マッチ取得失敗: {}", e),
                }
            });
        }
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
                                        <div class="content-header">
                                            <h2>
                                                {s.session_date.clone()}
                                                {s
                                                    .title
                                                    .as_ref()
                                                    .map(|t| format!(" {}", t))
                                                    .unwrap_or_default()}
                                            </h2>
                                            {s
                                                .notes
                                                .map(|note| {
                                                    view! { <div class="note">{note}</div> }
                                                })}

                                        </div>

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

#[component]
fn MatchList(
    session_id: i32,
    matches: ReadSignal<Vec<Match>>,
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_match_added: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (adding, set_adding) = signal(false);

    view! {
        <div class="match-list">
            {move || {
                matches
                    .get()
                    .iter()
                    .map(|m| {
                        view! {
                            <MatchItem
                                match_data=m.clone()
                                char_name=m.character_name.clone()
                                opp_name=m.opponent_character_name.clone()
                            />
                        }
                    })
                    .collect_view()
            }}

            <Show
                when=move || !adding.get()
                fallback=move || {
                    view! {
                        <InlineMatchForm
                            session_id=session_id
                            characters=characters
                            selected_character_id=selected_character_id
                            on_submit=move || {
                                set_adding.set(false);
                                on_match_added();
                            }

                            on_cancel=move || set_adding.set(false)
                        />
                    }
                }
            >

                <button class="add-match-button" on:click=move |_| set_adding.set(true)>
                    "+ マッチを追加"
                </button>
            </Show>
        </div>
    }
}

#[component]
fn MatchItem(match_data: Match, char_name: String, opp_name: String) -> impl IntoView {
    let initial_comment = match_data.comment.clone().unwrap_or_default();
    let (editing_comment, set_editing_comment) = signal(false);
    let (comment_value, set_comment_value) = signal(initial_comment.clone());

    let result_class = if match_data.result == "win" {
        "win"
    } else {
        "loss"
    };
    let result_symbol = if match_data.result == "win" {
        "○"
    } else {
        "×"
    };

    let match_id = match_data.id;

    let save_comment = move |should_close: bool| {
        let new_comment = comment_value.get();
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: None,
                opponent_character_id: None,
                result: None,
                comment: Some(new_comment),
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("コメント更新成功");
                    if should_close {
                        set_editing_comment.set(false);
                    }
                }
                Err(e) => {
                    logging::error!("コメント更新失敗: {}", e);
                }
            }
        });
    };

    view! {
        <div class="match-item">
            <div class="match-row">
                <div class="match-characters">{format!("{} vs {}", char_name, opp_name)}</div>

                <Show
                    when=move || editing_comment.get()
                    fallback=move || {
                        view! {
                            <div
                                class="match-comment editable"
                                on:click=move |_| set_editing_comment.set(true)
                            >
                                {move || {
                                    let c = comment_value.get();
                                    if c.is_empty() { "コメントを追加...".to_string() } else { c }
                                }}
                            </div>
                        }
                    }
                >
                    <input
                        type="text"
                        class="match-comment-input"
                        value=comment_value
                        on:input=move |ev| set_comment_value.set(event_target_value(&ev))
                        on:blur=move |_| {
                            save_comment(true);
                        }
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                save_comment(true);
                            } else if ev.key() == "Escape" {
                                set_editing_comment.set(false);
                            }
                        }
                        autofocus
                    />
                </Show>

                <div class=format!("match-result {}", result_class)>{result_symbol}</div>
            </div>
        </div>
    }
}

#[component]
fn InlineMatchForm(
    session_id: i32,
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_submit: impl Fn() + 'static + Copy,
    on_cancel: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let (character_id, set_character_id) = signal(selected_character_id.get().unwrap_or(0));
    let (opponent_id, set_opponent_id) = signal(0);
    let (result, set_result) = signal(String::from("win"));
    let (comment, set_comment) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        if character_id.get() == 0 || opponent_id.get() == 0 {
            logging::error!("使用キャラ、相手キャラを選択してください");
            return;
        }

        set_loading.set(true);

        let req = CreateMatchRequest {
            session_id,
            character_id: character_id.get(),
            opponent_character_id: opponent_id.get(),
            result: result.get(),
            comment: if comment.get().is_empty() {
                None
            } else {
                Some(comment.get())
            },
        };

        spawn_local(async move {
            match api::create_match(req).await {
                Ok(_) => {
                    logging::log!("マッチ記録成功");
                    on_submit();
                }
                Err(e) => {
                    logging::error!("マッチ記録失敗: {}", e);
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="add-match-form">
            <form on:submit=handle_submit>
                <div class="form-row">
                    <div class="form-group">
                        <label>"使用キャラ"</label>
                        <select
                            class="form-input"
                            on:change=move |ev| {
                                set_character_id.set(event_target_value(&ev).parse().unwrap_or(0))
                            }
                        >

                            <option value="0" selected=move || character_id.get() == 0>
                                "選択"
                            </option>
                            {move || {
                                characters
                                    .get()
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let is_selected = move || character_id.get() == char_id;
                                        view! {
                                            <option value=char_id selected=is_selected>
                                                {c.name.clone()}
                                            </option>
                                        }
                                    })
                                    .collect_view()
                            }}

                        </select>
                    </div>
                    <div class="form-group">
                        <label>"相手キャラ"</label>
                        <select
                            class="form-input"
                            on:change=move |ev| {
                                set_opponent_id.set(event_target_value(&ev).parse().unwrap_or(0))
                            }
                        >

                            <option value="0">"選択"</option>
                            {move || {
                                characters
                                    .get()
                                    .iter()
                                    .map(|c| view! { <option value=c.id>{c.name.clone()}</option> })
                                    .collect_view()
                            }}

                        </select>
                    </div>
                </div>

                <div class="form-group">
                    <label>"結果"</label>
                    <div class="radio-group">
                        <label class="radio-label">
                            <input
                                type="radio"
                                name="result"
                                value="win"
                                checked=move || result.get() == "win"
                                on:change=move |_| set_result.set("win".to_string())
                            />
                            " 勝ち"
                        </label>
                        <label class="radio-label">
                            <input
                                type="radio"
                                name="result"
                                value="loss"
                                checked=move || result.get() == "loss"
                                on:change=move |_| set_result.set("loss".to_string())
                            />
                            " 負け"
                        </label>
                    </div>
                </div>

                <div class="form-group">
                    <label>"コメント"</label>
                    <textarea
                        class="form-input"
                        placeholder="良い試合だった、など..."
                        on:input=move |ev| set_comment.set(event_target_value(&ev))
                        prop:value=comment
                    />
                </div>

                <div class="form-actions">
                    <button type="button" class="btn" on:click=move |_| on_cancel()>
                        "キャンセル"
                    </button>
                    <button type="submit" class="btn btn-primary" disabled=loading>
                        {move || if loading.get() { "記録中..." } else { "追加" }}
                    </button>
                </div>
            </form>
        </div>
    }
}

#[component]
fn Modal(on_close: impl Fn() + 'static + Copy, children: Children) -> impl IntoView {
    view! {
        <div class="modal-overlay" on:click=move |_| on_close()>
            <div class="modal-content" on:click=|e| e.stop_propagation()>
                <button class="modal-close" on:click=move |_| on_close()>
                    "×"
                </button>
                {children()}
            </div>
        </div>
    }
}

#[component]
fn SessionForm(on_submit: impl Fn() + 'static + Copy) -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (notes, set_notes) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);

        let title_value = title.get();
        let notes_value = notes.get();
        spawn_local(async move {
            let req = CreateSessionRequest {
                session_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
                title: if title_value.is_empty() {
                    None
                } else {
                    Some(title_value)
                },
                notes: if notes_value.is_empty() {
                    None
                } else {
                    Some(notes_value)
                },
            };

            match api::create_session(req).await {
                Ok(_) => {
                    logging::log!("セッション作成成功");
                    on_submit();
                }
                Err(e) => {
                    logging::error!("セッション作成失敗: {}", e);
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="form-container">
            <h2>"新しいセッション"</h2>
            <form on:submit=handle_submit>
                <div class="form-group">
                    <label>"日付"</label>
                    <input
                        type="date"
                        class="form-input"
                        value=chrono::Local::now().format("%Y-%m-%d").to_string()
                        disabled
                    />
                </div>
                <div class="form-group">
                    <label>"タイトル"</label>
                    <input
                        type="text"
                        class="form-input"
                        placeholder="今日の目標"
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        prop:value=title
                    />
                </div>
                <div class="form-group">
                    <label>"メモ"</label>
                    <textarea
                        class="form-input"
                        placeholder="気をつけること..."
                        on:input=move |ev| set_notes.set(event_target_value(&ev))
                        prop:value=notes
                    />
                </div>
                <div class="form-actions">
                    <button type="submit" class="btn btn-primary" disabled=loading>
                        {move || if loading.get() { "作成中..." } else { "作成" }}
                    </button>
                </div>
            </form>
        </div>
    }
}
