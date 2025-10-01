use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;

mod api;

use api::{Character, CreateMatchRequest, CreateSessionRequest, Session};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (show_session_modal, set_show_session_modal) = signal(false);
    let (show_match_modal, set_show_match_modal) = signal(false);
    let (sessions, set_sessions) = signal(Vec::<Session>::new());
    let (characters, set_characters) = signal(Vec::<Character>::new());
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

    let reload = move || {
        spawn_local(async move {
            if let Ok(data) = api::fetch_sessions().await {
                set_sessions.set(data);
            }
        });
    };

    view! {
        <div class="app">
            <Header/>
            <main class="container">
                <HomePage
                    sessions=sessions
                    loading=loading
                    on_new_session=move || set_show_session_modal.set(true)
                    on_new_match=move || set_show_match_modal.set(true)
                />
            </main>

            <Show when=move || show_session_modal.get()>
                <Modal on_close=move || set_show_session_modal.set(false)>
                    <SessionForm
                        on_submit=move || {
                            set_show_session_modal.set(false);
                            reload();
                        }
                    />
                </Modal>
            </Show>

            <Show when=move || show_match_modal.get()>
                <Modal on_close=move || set_show_match_modal.set(false)>
                    <MatchForm
                        sessions=sessions
                        characters=characters
                        on_submit=move || {
                            set_show_match_modal.set(false);
                            reload();
                        }
                    />
                </Modal>
            </Show>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <h1>"🎮 スマブラSP 戦績管理"</h1>
        </header>
    }
}

#[component]
fn HomePage(
    sessions: ReadSignal<Vec<Session>>,
    loading: ReadSignal<bool>,
    on_new_session: impl Fn() + 'static + Copy,
    on_new_match: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    view! {
        <div class="home">
            <section class="stats-card">
                <h2>"📊 今日の戦績"</h2>
                {move || {
                    if loading.get() {
                        view! { <p>"読み込み中..."</p> }.into_any()
                    } else {
                        let today_session = sessions.get().iter().find(|s| s.session_date == today).cloned();
                        match today_session {
                            Some(session) => {
                                view! {
                                    <div class="stats-grid">
                                        <div class="stat-item">
                                            <div class="stat-label">"試合数"</div>
                                            <div class="stat-value">{session.match_count}</div>
                                        </div>
                                        <div class="stat-item">
                                            <div class="stat-label">"勝ち"</div>
                                            <div class="stat-value">{session.wins}</div>
                                        </div>
                                        <div class="stat-item">
                                            <div class="stat-label">"負け"</div>
                                            <div class="stat-value">{session.losses}</div>
                                        </div>
                                        <div class="stat-item">
                                            <div class="stat-label">"勝率"</div>
                                            <div class="stat-value">
                                                {if session.match_count > 0 {
                                                    format!("{:.1}%", (session.wins as f64 / session.match_count as f64) * 100.0)
                                                } else {
                                                    "-".to_string()
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                }
                                    .into_any()
                            }
                            None => view! { <p>"今日のセッションはまだありません"</p> }.into_any(),
                        }
                    }
                }}

            </section>

            <section class="actions">
                <button class="btn btn-primary" on:click=move |_| on_new_session()>
                    "+ 新しいセッション"
                </button>
                <button class="btn btn-secondary" on:click=move |_| on_new_match()>
                    "+ マッチを記録"
                </button>
            </section>

            <section class="recent-sessions">
                <h2>"📅 最近のセッション"</h2>
                {move || {
                    if loading.get() {
                        view! { <p>"読み込み中..."</p> }.into_any()
                    } else {
                        view! {
                            <div class="session-list">
                                {sessions
                                    .get()
                                    .iter()
                                    .take(10)
                                    .map(|s| view! { <SessionCard session=s.clone()/> })
                                    .collect_view()}
                            </div>
                        }
                            .into_any()
                    }
                }}

            </section>
        </div>
    }
}

#[component]
fn SessionCard(session: Session) -> impl IntoView {
    view! {
        <div class="session-card">
            <div class="session-date">{session.session_date}</div>
            <div class="session-note">{session.notes.unwrap_or_else(|| "メモなし".to_string())}</div>
            <div class="session-matches">{format!("{} 勝 {} 敗", session.wins, session.losses)}</div>
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
fn MatchForm(
    sessions: ReadSignal<Vec<Session>>,
    characters: ReadSignal<Vec<Character>>,
    on_submit: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let (session_id, set_session_id) = signal(0);
    let (character_id, set_character_id) = signal(0);
    let (opponent_id, set_opponent_id) = signal(0);
    let (result, set_result) = signal(String::from("win"));
    let (gsp_before, set_gsp_before) = signal(String::new());
    let (gsp_after, set_gsp_after) = signal(String::new());
    let (comment, set_comment) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        if session_id.get() == 0 || character_id.get() == 0 || opponent_id.get() == 0 {
            logging::error!("セッション、使用キャラ、相手キャラを選択してください");
            return;
        }

        set_loading.set(true);

        let req = CreateMatchRequest {
            session_id: session_id.get(),
            character_id: character_id.get(),
            opponent_character_id: opponent_id.get(),
            result: result.get(),
            gsp_before: gsp_before.get().parse().ok(),
            gsp_after: gsp_after.get().parse().ok(),
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
        <div class="form-container">
            <h2>"マッチを記録"</h2>
            <form on:submit=handle_submit>
                <div class="form-group">
                    <label>"セッション"</label>
                    <select
                        class="form-input"
                        on:change=move |ev| set_session_id.set(event_target_value(&ev).parse().unwrap_or(0))
                    >
                        <option value="0">"選択してください"</option>
                        {move || {
                            sessions
                                .get()
                                .iter()
                                .map(|s| {
                                    view! {
                                        <option value=s.id>
                                            {format!(
                                                "{} - {}",
                                                s.session_date,
                                                s.notes.clone().unwrap_or_else(|| "メモなし".to_string()),
                                            )}

                                        </option>
                                    }
                                })
                                .collect_view()
                        }}

                    </select>
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label>"使用キャラ"</label>
                        <select
                            class="form-input"
                            on:change=move |ev| {
                                set_character_id.set(event_target_value(&ev).parse().unwrap_or(0))
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

                <div class="form-row">
                    <div class="form-group">
                        <label>"GSP（開始）"</label>
                        <input
                            type="number"
                            class="form-input"
                            placeholder="10000000"
                            on:input=move |ev| set_gsp_before.set(event_target_value(&ev))
                            prop:value=gsp_before
                        />
                    </div>
                    <div class="form-group">
                        <label>"GSP（終了）"</label>
                        <input
                            type="number"
                            class="form-input"
                            placeholder="10050000"
                            on:input=move |ev| set_gsp_after.set(event_target_value(&ev))
                            prop:value=gsp_after
                        />
                    </div>
                </div>

                <div class="form-group">
                    <label>"コメント"</label>
                    <textarea
                        class="form-input"
                        placeholder="良い試合だった、ラグかった、など..."
                        on:input=move |ev| set_comment.set(event_target_value(&ev))
                        prop:value=comment
                    />
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn btn-primary" disabled=loading>
                        {move || if loading.get() { "記録中..." } else { "記録" }}
                    </button>
                </div>
            </form>
        </div>
    }
}

#[component]
fn SessionForm(on_submit: impl Fn() + 'static + Copy) -> impl IntoView {
    let (notes, set_notes) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);

        let notes_value = notes.get();
        spawn_local(async move {
            let req = CreateSessionRequest {
                session_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
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
                    <label>"メモ"</label>
                    <textarea
                        class="form-input"
                        placeholder="今日の目標や気をつけること..."
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
