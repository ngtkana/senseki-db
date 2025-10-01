use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;

use crate::api::{self, Session};

#[component]
pub fn SessionHeader(
    session: Session,
    session_id: i32,
    on_session_updated: impl Fn() + 'static + Copy + Send + Sync,
    _version: ReadSignal<i32>,
) -> impl IntoView {
    let (editing_date, set_editing_date) = signal(false);
    let (editing_title, set_editing_title) = signal(false);
    let (editing_notes, set_editing_notes) = signal(false);

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
