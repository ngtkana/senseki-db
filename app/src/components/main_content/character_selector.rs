use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use crate::api::Character;
use crate::utils::character_search::matches_search;
use crate::utils::keyboard_navigation::handle_grid_keyboard_navigation;

#[component]
pub fn CharacterSelector(
    characters: Vec<Character>,
    selected_id: ReadSignal<Option<i32>>,
    on_select: impl Fn(i32) + 'static + Copy + Send + Sync,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] show_icon: bool,
    #[prop(optional)] auto_open: bool,
    #[prop(optional)] on_close: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = signal(false);
    let (search_query, set_search_query) = signal(String::new());
    let (dropdown_pos, set_dropdown_pos) = signal((0.0, 0.0, false)); // (left, top, show_above)
    let (cursor_index, set_cursor_index) = signal(0_i32);
    let input_ref = NodeRef::<leptos::html::Input>::new();
    let trigger_ref = NodeRef::<leptos::html::Div>::new();

    let placeholder_text = placeholder.unwrap_or("キャラ名で検索...");

    let characters_for_icon = characters.clone();
    let characters_for_display = characters.clone();

    // 選択されたキャラクター情報（アイコン用）
    let selected_character_for_icon = move || {
        selected_id
            .get()
            .and_then(|id| characters_for_icon.iter().find(|c| c.id == id).cloned())
    };

    // 選択されたキャラクター情報（表示用）
    let selected_character_for_display = move || {
        selected_id
            .get()
            .and_then(|id| characters_for_display.iter().find(|c| c.id == id).cloned())
    };

    // auto_open が true の場合、マウント時に自動的に開く
    Effect::new(move || {
        if auto_open && !show_dropdown.get() {
            if let Some(element) = trigger_ref.get() {
                let elem = element.as_ref() as &web_sys::Element;
                let rect = elem.get_bounding_client_rect();
                let window_height = web_sys::window()
                    .and_then(|w| w.inner_height().ok())
                    .and_then(|h| h.as_f64())
                    .unwrap_or(600.0);

                let dropdown_height = 500.0;
                let space_below = window_height - rect.bottom();
                let show_above = space_below < dropdown_height;
                let top = if show_above {
                    rect.top() - dropdown_height
                } else {
                    rect.bottom()
                };

                set_dropdown_pos.set((rect.left(), top, show_above));
                set_show_dropdown.set(true);
            }
        }
    });

    // ドロップダウンを開いたときに入力欄にフォーカス
    Effect::new(move || {
        if show_dropdown.get() {
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        } else {
            set_search_query.set(String::new());
            if let Some(ref callback) = on_close {
                callback();
            }
        }
    });

    // フィルタリング結果が変わったらカーソルを先頭にリセット
    Effect::new(move || {
        let _ = search_query.get(); // 依存関係を追加
        set_cursor_index.set(0);
    });

    let open_dropdown = move |ev: web_sys::MouseEvent| {
        if let Some(element) = ev.current_target() {
            if let Some(el) = element.dyn_ref::<web_sys::HtmlElement>() {
                let elem = el.as_ref() as &web_sys::Element;
                let rect = elem.get_bounding_client_rect();
                let window_height = web_sys::window()
                    .and_then(|w| w.inner_height().ok())
                    .and_then(|h| h.as_f64())
                    .unwrap_or(600.0);

                // ドロップダウンの高さ（最大500px）
                let dropdown_height = 500.0;
                let space_below = window_height - rect.bottom();

                // 下に十分なスペースがない場合は上に表示
                let show_above = space_below < dropdown_height;
                let top = if show_above {
                    rect.top() - dropdown_height
                } else {
                    rect.bottom()
                };

                set_dropdown_pos.set((rect.left(), top, show_above));
            }
        }
        set_show_dropdown.set(true);
    };

    let handle_select = move |char_id: i32| {
        on_select(char_id);
        set_show_dropdown.set(false);
    };

    view! {
        <div class="character-selector">
            {move || {
                if show_icon {
                    if let Some(char) = selected_character_for_icon() {
                        Some(view! {
                            <img
                                src=format!("/public/fighters/{}.png", char.fighter_key)
                                class="character-icon"
                                alt=char.name
                            />
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }}

            <div
                class="char-display editable"
                on:click=open_dropdown
                node_ref=trigger_ref
            >
                {move || {
                    if let Some(char) = selected_character_for_display() {
                        view! {
                            <>
                                <img
                                    src=format!("/public/fighters/{}.png", char.fighter_key)
                                    class="character-icon"
                                    alt=char.name.clone()
                                />
                            </>
                        }
                            .into_any()
                    } else {
                        view! { <div class="character-icon-placeholder">"？"</div> }.into_any()
                    }
                }}
            </div>

            <Show when=move || show_dropdown.get()>
                <div class="char-dropdown-overlay" on:click=move |_| set_show_dropdown.set(false)>
                    <div
                        class="character-dropdown-selector"
                        style=move || {
                            let (left, top, _) = dropdown_pos.get();
                            format!("left: {}px; top: {}px;", left, top)
                        }

                        on:click=|e| e.stop_propagation()
                    >
                        <div class="dropdown-header">"キャラを選択"</div>
                        <div class="dropdown-search">
                            <input
                                type="text"
                                class="search-input"
                                placeholder=placeholder_text
                                value=move || search_query.get()
                                on:input=move |ev| {
                                    set_search_query.set(event_target_value(&ev));
                                }

                                on:keydown={
                                    let characters = characters.clone();
                                    move |ev| {
                                        // Esc キーでドロップダウンを閉じる
                                        if ev.key() == "Escape" {
                                            ev.prevent_default();
                                            set_show_dropdown.set(false);
                                            return;
                                        }

                                        let query = search_query.get();
                                        let mut chars: Vec<_> = characters
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
                                                    handle_select(char.id);
                                                }
                                            },
                                        );
                                    }
                                }

                                node_ref=input_ref
                            />
                        </div>
                        <div class="character-grid">
                            {
                                let characters = characters.clone();
                                move || {
                                    let query = search_query.get();
                                    let mut chars: Vec<_> = characters
                                        .iter()
                                        .filter(|c| matches_search(c, &query))
                                        .cloned()
                                        .collect();
                                    chars.sort_by_key(|c| c.id);

                                    chars
                                        .iter()
                                        .enumerate()
                                        .map(|(index, c)| {
                                            let char_id = c.id;
                                            let fighter_key = c.fighter_key.clone();
                                            let char_name = c.name.clone();
                                            let char_name_for_alt = char_name.clone();
                                            let item_index = index as i32;
                                            view! {
                                                <div
                                                    class="character-grid-item"
                                                    class:selected=move || selected_id.get() == Some(char_id)
                                                    class:cursored=move || cursor_index.get() == item_index
                                                    on:click=move |_| {
                                                        handle_select(char_id);
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
                                }
                            }

                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
