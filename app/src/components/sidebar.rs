use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::web_sys;

use crate::api::{self, Session};

#[component]
pub fn Sidebar(
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
