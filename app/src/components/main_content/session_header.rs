use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;

use crate::api::{self, Session};
use crate::utils::gsp_format::{format_gsp, parse_gsp_input};

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
    let initial_start_gsp = session.start_gsp.map(format_gsp).unwrap_or_default();
    let initial_end_gsp = session.end_gsp.map(format_gsp).unwrap_or_default();

    let (date_value, set_date_value) = signal(initial_date.clone());
    let (title_value, set_title_value) = signal(initial_title.clone());
    let (notes_value, set_notes_value) = signal(initial_notes.clone());
    let (start_gsp_value, set_start_gsp_value) = signal(initial_start_gsp.clone());
    let (end_gsp_value, set_end_gsp_value) = signal(initial_end_gsp.clone());
    let (start_gsp_invalid, set_start_gsp_invalid) = signal(false);
    let (end_gsp_invalid, set_end_gsp_invalid) = signal(false);
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    // セッションが変わったら値をリセット
    Effect::new(move || {
        _version.track();
        set_date_value.set(session.session_date.clone());
        set_title_value.set(session.title.clone().unwrap_or_default());
        set_notes_value.set(session.notes.clone().unwrap_or_default());
        set_start_gsp_value.set(session.start_gsp.map(&format_gsp).unwrap_or_default());
        set_end_gsp_value.set(session.end_gsp.map(&format_gsp).unwrap_or_default());
        set_start_gsp_invalid.set(false);
        set_end_gsp_invalid.set(false);

        // textareaの値も直接更新
        if let Some(textarea) = textarea_ref.get() {
            if let Some(textarea_el) = textarea.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                textarea_el.set_value(&session.notes.clone().unwrap_or_default());
            }
        }
    });

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
        let numbers_only = parse_gsp_input(&new_gsp_str);

        // バリデーション: 空でない場合はパース可能かチェック
        if !numbers_only.is_empty() && numbers_only.parse::<i32>().is_err() {
            set_start_gsp_invalid.set(true);
            return;
        }

        set_start_gsp_invalid.set(false);
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: None,
                title: None,
                notes: None,
                start_gsp: Some(if numbers_only.is_empty() {
                    None
                } else {
                    numbers_only.parse::<i32>().ok()
                }),
                end_gsp: None,
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("開始GSP更新成功");
                    set_start_gsp_value.set(
                        updated_session
                            .start_gsp
                            .map(format_gsp)
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
        let numbers_only = parse_gsp_input(&new_gsp_str);

        // バリデーション: 空でない場合はパース可能かチェック
        if !numbers_only.is_empty() && numbers_only.parse::<i32>().is_err() {
            set_end_gsp_invalid.set(true);
            return;
        }

        set_end_gsp_invalid.set(false);
        spawn_local(async move {
            let req = api::UpdateSessionRequest {
                session_date: None,
                title: None,
                notes: None,
                start_gsp: None,
                end_gsp: Some(if numbers_only.is_empty() {
                    None
                } else {
                    numbers_only.parse::<i32>().ok()
                }),
            };
            match api::update_session(session_id, req).await {
                Ok(updated_session) => {
                    logging::log!("終了GSP更新成功");
                    set_end_gsp_value
                        .set(updated_session.end_gsp.map(format_gsp).unwrap_or_default());
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
                <span class="gsp-label">"開始時:"</span>
                <input
                    type="text"
                    class=move || if start_gsp_invalid.get() { "gsp-input gsp-input-invalid" } else { "gsp-input" }
                    value=start_gsp_value
                    on:input=move |ev| {
                        let val = event_target_value(&ev);

                        // 空文字列は許可
                        if val.is_empty() {
                            set_start_gsp_value.set(String::new());
                            set_start_gsp_invalid.set(false);
                            return;
                        }

                        // 数字とカンマのみ許可
                        if !val.chars().all(|c| c.is_numeric() || c == ',') {
                            set_start_gsp_invalid.set(true);
                            return;
                        }

                        // カンマを除去して数値のみ取得
                        let numbers_only: String = val.chars().filter(|c| c.is_numeric()).collect();

                        // パース可能かチェック
                        if let Ok(num) = numbers_only.parse::<i32>() {
                            // 正しいフォーマットに整形
                            let formatted = num.to_string()
                                .as_bytes()
                                .rchunks(3)
                                .rev()
                                .map(std::str::from_utf8)
                                .collect::<Result<Vec<&str>, _>>()
                                .unwrap()
                                .join(",");

                            // 入力値が正しいフォーマットかチェック
                            if val == formatted {
                                set_start_gsp_value.set(formatted);
                                set_start_gsp_invalid.set(false);
                            } else {
                                set_start_gsp_invalid.set(true);
                            }
                        } else {
                            set_start_gsp_invalid.set(true);
                        }
                    }
                    on:blur=move |_| save_start_gsp(false)
                />
                <span class="gsp-label">"終了時:"</span>
                <input
                    type="text"
                    class=move || if end_gsp_invalid.get() { "gsp-input gsp-input-invalid" } else { "gsp-input" }
                    value=end_gsp_value
                    on:input=move |ev| {
                        let val = event_target_value(&ev);

                        // 空文字列は許可
                        if val.is_empty() {
                            set_end_gsp_value.set(String::new());
                            set_end_gsp_invalid.set(false);
                            return;
                        }

                        // 数字とカンマのみ許可
                        if !val.chars().all(|c| c.is_numeric() || c == ',') {
                            set_end_gsp_invalid.set(true);
                            return;
                        }

                        // カンマを除去して数値のみ取得
                        let numbers_only: String = val.chars().filter(|c| c.is_numeric()).collect();

                        // パース可能かチェック
                        if let Ok(num) = numbers_only.parse::<i32>() {
                            // 正しいフォーマットに整形
                            let formatted = num.to_string()
                                .as_bytes()
                                .rchunks(3)
                                .rev()
                                .map(std::str::from_utf8)
                                .collect::<Result<Vec<&str>, _>>()
                                .unwrap()
                                .join(",");

                            // 入力値が正しいフォーマットかチェック
                            if val == formatted {
                                set_end_gsp_value.set(formatted);
                                set_end_gsp_invalid.set(false);
                            } else {
                                set_end_gsp_invalid.set(true);
                            }
                        } else {
                            set_end_gsp_invalid.set(true);
                        }
                    }
                    on:blur=move |_| save_end_gsp(false)
                />
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
