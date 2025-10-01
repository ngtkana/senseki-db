use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{self, CreateSessionRequest};

#[component]
pub fn Modal(on_close: impl Fn() + 'static + Copy, children: Children) -> impl IntoView {
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
pub fn SessionForm(on_submit: impl Fn() + 'static + Copy) -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (notes, set_notes) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let handle_submit = move |ev: leptos::web_sys::SubmitEvent| {
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
