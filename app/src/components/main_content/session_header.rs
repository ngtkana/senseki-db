use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;

use crate::api::{self, Session};

#[component]
pub fn SessionHeader(
    session: Session,
    session_id: i32,
    on_session_updated: impl Fn() + 'static + Copy + Send + Sync,
    _version: ReadSignal<i32>,
) -> impl IntoView {
    let initial_date = session.session_date.clone();
    let initial_title = session.title.clone().unwrap_or_default();
    let initial_notes = session.notes.clone().unwrap_or_default();
    let initial_start_gsp = session.start_gsp.map(|g| g.to_string()).unwrap_or_default();
    let initial_end_gsp = session.end_gsp.map(|g| g.to_string()).unwrap_or_default();

    let (date_value, set_date_value) = signal(initial_date.clone());
    let (title_value, set_title_value) = signal(initial_title.clone());
    let (notes_value, set_notes_value) = signal(initial_notes.clone());
    let (start_gsp_value, set_start_gsp_value) = signal(initial_start_gsp.clone());
    let (end_gsp_value, set_end_gsp_value) = signal(initial_end_gsp.clone());
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    // 初期表示時とnotes_valueが変更されたときに高さを調整
    Effect::new(move || {
        notes_value.track();
        if let Some(textarea) = textarea_ref.get() {
            let html_element: &web_sys::HtmlElement = textarea.unchecked_ref();
            let _ = html_element.style().set_property("height", "auto");
            if let Some(textarea_el) = textarea.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                let scroll_height = textarea_el.scroll_height();
                let _ = html_element
                    .style()
                    .set_property("height", &format!("{}px", scroll_height));
            }
        }
    });

    let save_date = move |_should_close: bool| {
        let new_date = date_value.get();
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: Some(new_date.clone()),
                title: None,
                notes: None,
                start_gsp: None,
                end_gsp: None,
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("日付更新成功");
                    set_date_value.set(updated_session.session_date);
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("日付更新失敗: {}", e);
                }
            }
        });
    };

    let save_title = move |_should_close: bool| {
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
                start_gsp: None,
                end_gsp: None,
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("タイトル更新成功");
                    set_title_value.set(updated_session.title.unwrap_or_default());
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("タイトル更新失敗: {}", e);
                }
            }
        });
    };

    let save_notes = move |_should_close: bool| {
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
                start_gsp: None,
                end_gsp: None,
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("コメント更新成功");
                    set_notes_value.set(updated_session.notes.unwrap_or_default());
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("コメント更新失敗: {}", e);
                }
            }
        });
    };

    let save_start_gsp = move |_should_close: bool| {
        let new_gsp_str = start_gsp_value.get();
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: None,
                title: None,
                notes: None,
                start_gsp: Some(if new_gsp_str.is_empty() {
                    None
                } else {
                    new_gsp_str.parse::<i32>().ok()
                }),
                end_gsp: None,
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("開始GSP更新成功");
                    set_start_gsp_value.set(
                        updated_session
                            .start_gsp
                            .map(|g| g.to_string())
                            .unwrap_or_default(),
                    );
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("開始GSP更新失敗: {}", e);
                }
            }
        });
    };

    let save_end_gsp = move |_should_close: bool| {
        let new_gsp_str = end_gsp_value.get();
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: None,
                title: None,
                notes: None,
                start_gsp: None,
                end_gsp: Some(if new_gsp_str.is_empty() {
                    None
                } else {
                    new_gsp_str.parse::<i32>().ok()
                }),
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("終了GSP更新成功");
                    set_end_gsp_value.set(
                        updated_session
                            .end_gsp
                            .map(|g| g.to_string())
                            .unwrap_or_default(),
                    );
                    on_session_updated();
                }
                Err(e) => {
                    logging::error!("終了GSP更新失敗: {}", e);
                }
            }
        });
    };

    view! {
        <div class="content-header">
            <div class="session-title-row">
                <input
                    type="date"
                    class="session-date-input"
                    value=date_value
                    on:input=move |ev| set_date_value.set(event_target_value(&ev))
                    on:blur=move |_| save_date(false)
                />

                <input
                    type="text"
                    class="session-title-input"
                    value=title_value
                    on:input=move |ev| set_title_value.set(event_target_value(&ev))
                    on:blur=move |_| save_title(false)
                />
            </div>

            <div class="session-gsp-row">
                <div class="gsp-input-group">
                    <label>"開始GSP:"</label>
                    <input
                        type="number"
                        class="gsp-input"
                        placeholder="例: 12000000"
                        value=start_gsp_value
                        on:input=move |ev| set_start_gsp_value.set(event_target_value(&ev))
                        on:blur=move |_| save_start_gsp(false)
                    />
                </div>
                <div class="gsp-input-group">
                    <label>"終了GSP:"</label>
                    <input
                        type="number"
                        class="gsp-input"
                        placeholder="例: 12500000"
                        value=end_gsp_value
                        on:input=move |ev| set_end_gsp_value.set(event_target_value(&ev))
                        on:blur=move |_| save_end_gsp(false)
                    />
                </div>
            </div>

            <textarea
                class="session-notes-input"
                prop:value=notes_value
                on:input=move |ev| {
                    set_notes_value.set(event_target_value(&ev));
                }
                on:blur=move |_| save_notes(false)
                node_ref=textarea_ref
            />
        </div>
    }
}
