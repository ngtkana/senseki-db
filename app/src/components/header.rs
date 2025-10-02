use leptos::prelude::*;

use crate::api::{Character, Session};
use crate::components::common::character_icon::{
    CharacterIcon, CharacterIconPlaceholder, IconSize,
};
use crate::utils::character_search::matches_search;
use crate::utils::keyboard_navigation::handle_grid_keyboard_navigation;

#[component]
pub fn Header(
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_character_select: impl Fn(i32) + 'static + Copy + Send + Sync,
    sessions: ReadSignal<Vec<Session>>,
    selected_session_id: ReadSignal<Option<i32>>,
    on_session_prev: impl Fn() + 'static + Copy + Send + Sync,
    on_session_next: impl Fn() + 'static + Copy + Send + Sync,
    on_session_add: impl Fn() + 'static + Copy + Send + Sync,
    on_session_delete: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);
    let (search_query, set_search_query) = signal(String::new());
    let (cursor_index, set_cursor_index) = signal(0_i32);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let selected_character = move || {
        selected_character_id
            .get()
            .and_then(|id| characters.get().iter().find(|c| c.id == id).cloned())
    };

    // フィルタリングされたキャラクターリスト
    let filtered_characters = move || {
        let query = search_query.get();
        let chars = characters.get();
        let filtered: Vec<_> = chars
            .iter()
            .filter(|c| matches_search(c, &query))
            .cloned()
            .collect();
        filtered
    };

    // ドロップダウンを開いたときに入力欄にフォーカス
    Effect::new(move || {
        if show_dropdown.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        } else {
            set_search_query.set(String::new());
        }
    });

    // フィルタリング結果が変わったらカーソルを先頭にリセット
    Effect::new(move || {
        let _ = search_query.get(); // 依存関係を追加
        set_cursor_index.set(0);
    });

    // セッション操作ボタンの有効/無効判定
    let can_go_prev = move || {
        let current_id = selected_session_id.get();
        let all_sessions = sessions.get();
        if let Some(current) = current_id {
            if let Some(current_index) = all_sessions.iter().position(|s| s.id == current) {
                return current_index + 1 < all_sessions.len();
            }
        }
        false
    };

    let can_go_next = move || {
        let current_id = selected_session_id.get();
        let all_sessions = sessions.get();
        if let Some(current) = current_id {
            if let Some(current_index) = all_sessions.iter().position(|s| s.id == current) {
                return current_index > 0;
            }
        }
        false
    };

    view! {
        <header class="header">
            <h1>"スマブラSP 戦績管理"</h1>

            <div class="header-session-controls">
                <button
                    class="session-control-btn"
                    disabled=move || !can_go_prev()
                    on:click=move |_| on_session_prev()
                    title="前のセッション"
                >
                    "◀"
                </button>
                <button
                    class="session-control-btn"
                    disabled=move || !can_go_next()
                    on:click=move |_| on_session_next()
                    title="次のセッション"
                >
                    "▶"
                </button>
                <button
                    class="session-control-btn session-add-btn"
                    on:click=move |_| on_session_add()
                    title="新しいセッション"
                >
                    "+"
                </button>
                <button
                    class="session-control-btn session-delete-btn"
                    disabled=move || selected_session_id.get().is_none()
                    on:click=move |_| set_show_delete_confirm.set(true)
                    title="セッションを削除"
                >
                    "🗑"
                </button>
            </div>

            <div class="header-character">
                <div
                    class="character-avatar"
                    on:click=move |_| set_show_dropdown.set(!show_dropdown.get())
                >
                    {move || {
                        if let Some(char) = selected_character() {
                            view! {
                                <CharacterIcon
                                    fighter_key=char.fighter_key
                                    alt=char.name
                                    size=IconSize::Large
                                />
                            }
                                .into_any()
                        } else {
                            view! {
                                <CharacterIconPlaceholder size=IconSize::Large/>
                            }
                                .into_any()
                        }
                    }}

                </div>

                <Show when=move || show_dropdown.get()>
                    <div class="char-dropdown-overlay" on:click=move |_| set_show_dropdown.set(false)>
                        <div
                            class="character-dropdown"
                            on:click=move |e| e.stop_propagation()
                        >
                            <div class="dropdown-header">"使用キャラを選択"</div>
                            <div class="dropdown-search">
                                <input
                                    type="text"
                                    class="search-input"
                                    placeholder="キャラ名で検索..."
                                    value=move || search_query.get()
                                    on:input=move |ev| {
                                        set_search_query.set(event_target_value(&ev));
                                    }

                                    on:keydown=move |ev| {
                                        let query = search_query.get();
                                        let chars_vec = characters.get();
                                        let mut chars: Vec<_> = chars_vec
                                            .iter()
                                            .filter(|c| matches_search(c, &query))
                                            .cloned()
                                            .collect();
                                        chars.sort_by_key(|c| c.id);
                                        let char_count = chars.len() as i32;
                                        let current_index = cursor_index.get();

                                        handle_grid_keyboard_navigation(
                                            &ev,
                                            current_index,
                                            char_count,
                                            8,
                                            move |idx| set_cursor_index.set(idx),
                                            move || {
                                                if let Some(char) = chars.get(current_index as usize) {
                                                    on_character_select(char.id);
                                                    set_show_dropdown.set(false);
                                                }
                                            },
                                        );
                                    }

                                    node_ref=input_ref
                                />
                            </div>
                            <div class="character-grid">
                            {move || {
                                let mut chars = filtered_characters();
                                chars.sort_by_key(|c| c.id);
                                chars
                                    .iter()
                                    .enumerate()
                                    .map(|(index, c)| {
                                        let char_id = c.id;
                                        let is_selected = selected_character_id.get() == Some(char_id);
                                        let fighter_key = c.fighter_key.clone();
                                        let char_name = c.name.clone();
                                        let char_name_for_alt = char_name.clone();
                                        let item_index = index as i32;
                                        view! {
                                            <div
                                                class="character-grid-item"
                                                class:selected=move || is_selected
                                                class:cursored=move || cursor_index.get() == item_index
                                                on:click=move |_| {
                                                    on_character_select(char_id);
                                                    set_show_dropdown.set(false);
                                                }

                                                title=char_name
                                            >
                                                <CharacterIcon
                                                    fighter_key=fighter_key
                                                    alt=char_name_for_alt
                                                    size=IconSize::Large
                                                />
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}

                            </div>
                        </div>
                    </div>
                </Show>
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
                            <button
                                class="btn btn-danger"
                                on:click=move |_| {
                                    on_session_delete();
                                    set_show_delete_confirm.set(false);
                                }
                            >
                                "削除"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </header>
    }
}
