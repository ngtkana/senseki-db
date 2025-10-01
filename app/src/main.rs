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
    on_character_select: impl Fn(i32) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = signal(false);

    let selected_character = move || {
        selected_character_id
            .get()
            .and_then(|id| characters.get().iter().find(|c| c.id == id).cloned())
    };

    view! {
        <header class="header">
            <h1>"スマブラSP 戦績管理"</h1>
            <div class="header-character">
                <div
                    class="character-avatar"
                    on:click=move |_| set_show_dropdown.set(!show_dropdown.get())
                >
                    {move || {
                        if let Some(char) = selected_character() {
                            view! {
                                <img
                                    src=format!("/public/fighters/{}.png", char.fighter_key)
                                    class="avatar-icon"
                                    alt=char.name
                                />
                            }
                                .into_any()
                        } else {
                            view! { <div class="avatar-placeholder">"?"</div> }.into_any()
                        }
                    }}

                </div>

                <Show when=move || show_dropdown.get()>
                    <div
                        class="character-dropdown"
                        on:click=move |e| e.stop_propagation()
                    >
                        <div class="dropdown-header">"使用キャラを選択"</div>
                        <div class="character-grid">
                            {move || {
                                let mut chars = characters.get();
                                chars.sort_by_key(|c| c.id);
                                chars
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let is_selected = selected_character_id.get() == Some(char_id);
                                        let fighter_key = c.fighter_key.clone();
                                        let char_name = c.name.clone();
                                        let char_name_for_alt = char_name.clone();
                                        view! {
                                            <div
                                                class="character-grid-item"
                                                class:selected=move || is_selected
                                                on:click=move |_| {
                                                    on_character_select(char_id);
                                                    set_show_dropdown.set(false);
                                                }

                                                title=char_name
                                            >
                                                <img
                                                    src=format!("/public/fighters/{}.png", fighter_key)
                                                    class="grid-icon"
                                                    alt=char_name_for_alt
                                                />
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}

                        </div>
                    </div>
                </Show>
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
    on_session_deleted: impl Fn() + 'static + Copy + Send + Sync,
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
                                        <SessionItem
                                            session=session.clone()
                                            is_selected=is_selected
                                            on_select=move || on_select(session_id)
                                            on_deleted=on_session_deleted
                                        />
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
fn SessionItem(
    session: Session,
    is_selected: impl Fn() -> bool + 'static + Copy + Send,
    on_select: impl Fn() + 'static + Copy,
    on_deleted: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (show_menu, set_show_menu) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);
    let session_id = session.id;

    let handle_delete = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
        set_show_delete_confirm.set(true);
        set_show_menu.set(false);
    };

    let confirm_delete = move || {
        spawn_local(async move {
            match api::delete_session(session_id).await {
                Ok(_) => {
                    logging::log!("セッション削除成功");
                    on_deleted();
                }
                Err(e) => {
                    logging::error!("セッション削除失敗: {}", e);
                }
            }
        });
    };

    view! {
        <>
            <div class=move || if is_selected() { "session-item selected" } else { "session-item" } on:click=move |_| on_select()>
                <div class="session-date">
                    {session.session_date.clone()}
                    {session.title.as_ref().map(|t| format!(" {}", t)).unwrap_or_default()}
                </div>
                <div class="session-item-menu">
                    <button
                        class="menu-button"
                        on:click=move |e| {
                            e.stop_propagation();
                            set_show_menu.set(!show_menu.get());
                        }
                    >
                        "⋮"
                    </button>
                    <Show when=move || show_menu.get()>
                        <div class="menu-dropdown">
                            <button class="menu-item delete" on:click=handle_delete>
                                "削除"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>

            <Show when=move || show_delete_confirm.get()>
                <div class="confirm-overlay" on:click=move |_| set_show_delete_confirm.set(false)>
                    <div class="confirm-dialog" on:click=|e| e.stop_propagation()>
                        <h3>"セッションを削除しますか？"</h3>
                        <p>"このセッションに含まれる全てのマッチも削除されます。"</p>
                        <div class="confirm-actions">
                            <button
                                class="btn"
                                on:click=move |_| set_show_delete_confirm.set(false)
                            >
                                "キャンセル"
                            </button>
                            <button class="btn btn-danger" on:click=move |_| confirm_delete()>
                                "削除"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </>
    }
}

#[component]
fn MainContent(
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

#[component]
fn SessionHeader(
    session: Session,
    session_id: i32,
    on_session_updated: impl Fn() + 'static + Copy + Send + Sync,
    _version: ReadSignal<i32>,
) -> impl IntoView {
    let (editing_date, set_editing_date) = signal(false);
    let (editing_title, set_editing_title) = signal(false);
    let (editing_notes, set_editing_notes) = signal(false);

    // 初期値を設定
    let initial_date = session.session_date.clone();
    let initial_title = session.title.clone().unwrap_or_default();
    let initial_notes = session.notes.clone().unwrap_or_default();

    let (date_value, set_date_value) = signal(initial_date.clone());
    let (title_value, set_title_value) = signal(initial_title.clone());
    let (notes_value, set_notes_value) = signal(initial_notes.clone());

    let save_date = move |should_close: bool| {
        let new_date = date_value.get();
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: Some(new_date.clone()),
                title: None,
                notes: None,
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("日付更新成功");
                    set_date_value.set(updated_session.session_date);
                    if should_close {
                        set_editing_date.set(false);
                    }
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("日付更新失敗: {}", e);
                }
            }
        });
    };

    let save_title = move |should_close: bool| {
        let new_title = title_value.get();
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: None,
                title: Some(if new_title.is_empty() {
                    None
                } else {
                    Some(new_title.clone())
                }),
                notes: None,
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("タイトル更新成功");
                    set_title_value.set(updated_session.title.unwrap_or_default());
                    if should_close {
                        set_editing_title.set(false);
                    }
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("タイトル更新失敗: {}", e);
                }
            }
        });
    };

    let save_notes = move |should_close: bool| {
        let new_notes = notes_value.get();
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: None,
                title: None,
                notes: Some(if new_notes.is_empty() {
                    None
                } else {
                    Some(new_notes.clone())
                }),
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("コメント更新成功");
                    set_notes_value.set(updated_session.notes.unwrap_or_default());
                    if should_close {
                        set_editing_notes.set(false);
                    }
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("コメント更新失敗: {}", e);
                }
            }
        });
    };

    view! {
        <div class="content-header">
            <div class="session-title-row">
                <Show
                    when=move || editing_date.get()
                    fallback=move || {
                        view! {
                            <span
                                class="session-date-label editable"
                                on:click=move |_| set_editing_date.set(true)
                            >
                                {date_value}
                            </span>
                        }
                    }
                >
                    <input
                        type="date"
                        class="session-date-input"
                        value=date_value
                        on:input=move |ev| set_date_value.set(event_target_value(&ev))
                        on:blur=move |_| {
                            save_date(true);
                        }
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                save_date(true);
                            } else if ev.key() == "Escape" {
                                set_editing_date.set(false);
                            }
                        }
                        autofocus
                    />
                </Show>

                <Show
                    when=move || editing_title.get()
                    fallback=move || {
                        view! {
                            <div
                                class="session-title-edit editable"
                                on:click=move |_| set_editing_title.set(true)
                            >
                                {move || {
                                    let t = title_value.get();
                                    if t.is_empty() { "タイトルを追加...".to_string() } else { t }
                                }}
                            </div>
                        }
                    }
                >
                    <input
                        type="text"
                        class="session-title-input"
                        value=title_value
                        on:input=move |ev| set_title_value.set(event_target_value(&ev))
                        on:blur=move |_| {
                            save_title(true);
                        }
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                save_title(true);
                            } else if ev.key() == "Escape" {
                                set_editing_title.set(false);
                            }
                        }
                        autofocus
                    />
                </Show>
            </div>

            <Show
                when=move || editing_notes.get()
                fallback=move || {
                    view! {
                        <div
                            class="session-notes-edit editable"
                            on:click=move |_| set_editing_notes.set(true)
                        >
                            {move || {
                                let n = notes_value.get();
                                if n.is_empty() { "コメントを追加...".to_string() } else { n }
                            }}
                        </div>
                    }
                }
            >
                <textarea
                    class="session-notes-input"
                    prop:value=notes_value
                    on:input=move |ev| set_notes_value.set(event_target_value(&ev))
                    on:blur=move |_| {
                        save_notes(true);
                    }
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        if ev.key() == "Escape" {
                            set_editing_notes.set(false);
                        }
                    }
                    autofocus
                />
            </Show>

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
                let chars = characters.get();
                matches
                    .get()
                    .iter()
                    .map(|m| {
                        view! {
                            <MatchItem
                                match_data=m.clone()
                                char_name=m.character_name.clone()
                                opp_name=m.opponent_character_name.clone()
                                characters=chars.clone()
                                on_match_deleted=on_match_added
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
fn MatchItem(
    match_data: Match,
    char_name: String,
    opp_name: String,
    characters: Vec<Character>,
    on_match_deleted: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let initial_comment = match_data.comment.clone().unwrap_or_default();
    let (editing_comment, set_editing_comment) = signal(false);
    let (editing_char, set_editing_char) = signal(false);
    let (editing_opp, set_editing_opp) = signal(false);
    let (editing_result, set_editing_result) = signal(false);
    let (comment_value, set_comment_value) = signal(initial_comment.clone());
    let (show_menu, set_show_menu) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);

    // キャラクターIDを取得
    let char_id = characters
        .iter()
        .find(|c| c.name == char_name)
        .map(|c| c.id)
        .unwrap_or(0);
    let opp_id = characters
        .iter()
        .find(|c| c.name == opp_name)
        .map(|c| c.id)
        .unwrap_or(0);

    let (selected_char_id, set_selected_char_id) = signal(char_id);
    let (selected_opp_id, set_selected_opp_id) = signal(opp_id);
    let (result_value, set_result_value) = signal(match_data.result.clone());

    let result_class = move || {
        if result_value.get() == "win" {
            "win"
        } else {
            "loss"
        }
    };
    let result_symbol = move || {
        if result_value.get() == "win" {
            "○"
        } else {
            "×"
        }
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

    let save_character = move |new_char_id: i32| {
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: Some(new_char_id),
                opponent_character_id: None,
                result: None,
                comment: None,
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("自キャラ更新成功");
                    set_editing_char.set(false);
                }
                Err(e) => {
                    logging::error!("自キャラ更新失敗: {}", e);
                }
            }
        });
    };

    let save_opponent = move |new_opp_id: i32| {
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: None,
                opponent_character_id: Some(new_opp_id),
                result: None,
                comment: None,
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("相手キャラ更新成功");
                    set_editing_opp.set(false);
                }
                Err(e) => {
                    logging::error!("相手キャラ更新失敗: {}", e);
                }
            }
        });
    };

    let save_result = move |new_result: String| {
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: None,
                opponent_character_id: None,
                result: Some(new_result.clone()),
                comment: None,
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("勝敗更新成功");
                    set_result_value.set(new_result);
                    set_editing_result.set(false);
                }
                Err(e) => {
                    logging::error!("勝敗更新失敗: {}", e);
                }
            }
        });
    };

    // キャラクター名からfighter_keyを取得
    let characters_for_char_info = characters.clone();
    let get_char_info = move |id: i32| {
        characters_for_char_info
            .iter()
            .find(|c| c.id == id)
            .map(|c| (c.name.clone(), c.fighter_key.clone()))
            .unwrap_or_default()
    };

    let characters_for_opp_info = characters.clone();
    let get_opp_info = move |id: i32| {
        characters_for_opp_info
            .iter()
            .find(|c| c.id == id)
            .map(|c| (c.name.clone(), c.fighter_key.clone()))
            .unwrap_or_default()
    };

    let characters_for_char_select = characters.clone();
    let characters_for_opp_select = characters.clone();

    let handle_delete = move || {
        set_show_delete_confirm.set(true);
        set_show_menu.set(false);
    };

    let confirm_delete = move || {
        spawn_local(async move {
            match api::delete_match(match_id).await {
                Ok(_) => {
                    logging::log!("マッチ削除成功");
                    on_match_deleted();
                }
                Err(e) => {
                    logging::error!("マッチ削除失敗: {}", e);
                }
            }
        });
    };

    view! {
        <div class="match-item">
            <div class="match-row">
                <div class="match-characters">
                    <Show
                        when=move || editing_char.get()
                        fallback=move || {
                            let (name, key) = get_char_info(selected_char_id.get());
                            view! {
                                <div
                                    class="char-display editable"
                                    on:click=move |_| set_editing_char.set(true)
                                >
                                    <img
                                        src=format!("/public/fighters/{}.png", key)
                                        class="character-icon"
                                        alt=name.clone()
                                    />
                                    <span>{name}</span>
                                </div>
                            }
                        }
                    >

                        <select
                            class="char-select"
                            on:change=move |ev| {
                                let new_id = event_target_value(&ev).parse().unwrap_or(0);
                                set_selected_char_id.set(new_id);
                                save_character(new_id);
                            }

                            on:blur=move |_| set_editing_char.set(false)
                            autofocus
                        >
                            {characters_for_char_select
                                .iter()
                                .map(|c| {
                                    let char_id = c.id;
                                    let is_selected = selected_char_id.get() == char_id;
                                    view! {
                                        <option value=char_id selected=is_selected>
                                            {c.name.clone()}
                                        </option>
                                    }
                                })
                                .collect_view()}

                        </select>
                    </Show>

                    <span class="vs-text">" vs "</span>

                    <Show
                        when=move || editing_opp.get()
                        fallback=move || {
                            let (name, key) = get_opp_info(selected_opp_id.get());
                            view! {
                                <div
                                    class="char-display editable"
                                    on:click=move |_| set_editing_opp.set(true)
                                >
                                    <img
                                        src=format!("/public/fighters/{}.png", key)
                                        class="character-icon"
                                        alt=name.clone()
                                    />
                                    <span>{name}</span>
                                </div>
                            }
                        }
                    >

                        <select
                            class="char-select"
                            on:change=move |ev| {
                                let new_id = event_target_value(&ev).parse().unwrap_or(0);
                                set_selected_opp_id.set(new_id);
                                save_opponent(new_id);
                            }

                            on:blur=move |_| set_editing_opp.set(false)
                            autofocus
                        >
                            {characters_for_opp_select
                                .iter()
                                .map(|c| {
                                    let char_id = c.id;
                                    let is_selected = selected_opp_id.get() == char_id;
                                    view! {
                                        <option value=char_id selected=is_selected>
                                            {c.name.clone()}
                                        </option>
                                    }
                                })
                                .collect_view()}

                        </select>
                    </Show>
                </div>

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

                <Show
                    when=move || editing_result.get()
                    fallback=move || {
                        view! {
                            <div
                                class=move || format!("match-result {} editable", result_class())
                                on:click=move |_| set_editing_result.set(true)
                            >
                                {result_symbol}
                            </div>
                        }
                    }
                >

                    <select
                        class="result-select"
                        on:change=move |ev| {
                            let new_result = event_target_value(&ev);
                            save_result(new_result);
                        }

                        on:blur=move |_| set_editing_result.set(false)
                        autofocus
                    >
                        <option value="win" selected=move || result_value.get() == "win">
                            "勝ち"
                        </option>
                        <option value="loss" selected=move || result_value.get() == "loss">
                            "負け"
                        </option>
                    </select>
                </Show>

                <div class="match-menu">
                    <button
                        class="menu-button"
                        on:click=move |_| set_show_menu.set(!show_menu.get())
                    >
                        "⋮"
                    </button>
                    <Show when=move || show_menu.get()>
                        <div class="menu-dropdown">
                            <button class="menu-item delete" on:click=move |_| handle_delete()>
                                "削除"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>

            <Show when=move || show_delete_confirm.get()>
                <div class="confirm-overlay" on:click=move |_| set_show_delete_confirm.set(false)>
                    <div class="confirm-dialog" on:click=|e| e.stop_propagation()>
                        <h3>"マッチを削除しますか？"</h3>
                        <p>"この操作は取り消せません。"</p>
                        <div class="confirm-actions">
                            <button
                                class="btn"
                                on:click=move |_| set_show_delete_confirm.set(false)
                            >
                                "キャンセル"
                            </button>
                            <button class="btn btn-danger" on:click=move |_| confirm_delete()>
                                "削除"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
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
