use gloo_timers::future::sleep;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use std::time::Duration;

use crate::api::Character;

fn normalize_for_search(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

fn matches_search(character: &Character, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let normalized_query = normalize_for_search(query);

    // 日本語名で検索
    if normalize_for_search(&character.name).contains(&normalized_query) {
        return true;
    }

    // 英語名で検索
    if normalize_for_search(&character.name_en).contains(&normalized_query) {
        return true;
    }

    // fighter_keyでも検索（内部キー）
    if normalize_for_search(&character.fighter_key).contains(&normalized_query) {
        return true;
    }

    // TODO: ローマ字検索を実装する場合はここに追加

    false
}

#[component]
pub fn CharacterAutocomplete(
    characters: Vec<Character>,
    selected_id: Option<i32>,
    on_select: impl Fn(i32) + 'static + Copy + Send + Sync,
    placeholder: &'static str,
) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());
    let (is_focused, set_is_focused) = signal(false);
    let (selected_index, set_selected_index) = signal(0usize);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let characters_for_display = characters.clone();
    let characters_for_filter = characters.clone();
    let characters_for_keydown = characters.clone();

    // 選択されたキャラクターの名前を表示
    let display_name = move || {
        if let Some(id) = selected_id {
            characters_for_display
                .iter()
                .find(|c| c.id == id)
                .map(|c| c.name.clone())
                .unwrap_or_default()
        } else {
            String::new()
        }
    };

    // フィルタリングされたキャラクターリスト
    let filtered_characters = move || {
        let query = input_value.get();
        let mut filtered: Vec<_> = characters_for_filter
            .iter()
            .filter(|c| matches_search(c, &query))
            .cloned()
            .collect();
        filtered.sort_by_key(|c| c.id);
        filtered
    };

    let handle_select = move |char_id: i32| {
        on_select(char_id);
        set_is_focused.set(false);
        set_input_value.set(String::new());
        set_selected_index.set(0);
    };

    let filtered_characters_2 = move || {
        let query = input_value.get();
        let mut filtered: Vec<_> = characters_for_keydown
            .iter()
            .filter(|c| matches_search(c, &query))
            .cloned()
            .collect();
        filtered.sort_by_key(|c| c.id);
        filtered
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        let filtered = filtered_characters_2();

        match key.as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                if !filtered.is_empty() {
                    set_selected_index.update(|idx| {
                        *idx = (*idx + 1).min(filtered.len() - 1);
                    });
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| {
                    *idx = idx.saturating_sub(1);
                });
            }
            "Enter" => {
                ev.prevent_default();
                let idx = selected_index.get();
                if let Some(character) = filtered.get(idx) {
                    handle_select(character.id);
                }
            }
            "Escape" => {
                set_is_focused.set(false);
                set_input_value.set(String::new());
                set_selected_index.set(0);
            }
            _ => {}
        }
    };

    let characters_for_icon = characters.clone();
    let display_icon = move || {
        if let Some(id) = selected_id {
            characters_for_icon
                .iter()
                .find(|c| c.id == id)
                .map(|c| c.fighter_key.clone())
        } else {
            None
        }
    };

    let (dropdown_position, set_dropdown_position) = signal("bottom".to_string());
    let wrapper_ref = NodeRef::<leptos::html::Div>::new();

    // フォーカス時にドロップダウンの位置を計算
    let calculate_position = move || {
        if let Some(wrapper) = wrapper_ref.get() {
            let rect = wrapper.get_bounding_client_rect();
            let window_height = web_sys::window()
                .and_then(|w| w.inner_height().ok())
                .and_then(|h| h.as_f64())
                .unwrap_or(600.0);

            let dropdown_height = 300.0;
            let space_below = window_height - rect.bottom();

            if space_below < dropdown_height {
                set_dropdown_position.set("top".to_string());
            } else {
                set_dropdown_position.set("bottom".to_string());
            }
        }
    };

    view! {
        <div class="character-autocomplete" node_ref=wrapper_ref>
            {move || {
                if let Some(key) = display_icon() {
                    Some(view! {
                        <img
                            src=format!("/public/fighters/{}.png", key)
                            class="autocomplete-selected-icon"
                            alt=""
                        />
                    })
                } else {
                    None
                }
            }}
            <div class="autocomplete-input-wrapper">
                <input
                    type="text"
                    class="autocomplete-input"
                    placeholder=placeholder
                    value=move || {
                        if is_focused.get() {
                            input_value.get()
                        } else {
                            display_name()
                        }
                    }
                    on:input=move |ev| {
                        set_input_value.set(event_target_value(&ev));
                        set_selected_index.set(0);
                    }
                    on:focus=move |_| {
                        calculate_position();
                        set_is_focused.set(true);
                        set_input_value.set(String::new());
                    }
                    on:blur=move |_| {
                        // 少し遅延させてクリックイベントを処理できるようにする
                        leptos::task::spawn_local(async move {
                            sleep(Duration::from_millis(200)).await;
                            set_is_focused.set(false);
                            set_input_value.set(String::new());
                            set_selected_index.set(0);
                        });
                    }
                    on:keydown=handle_keydown
                    node_ref=input_ref
                />
            </div>

            {move || {
                let is_focused_val = is_focused.get();
                let filtered = filtered_characters();
                let has_items = !filtered.is_empty();

                if is_focused_val && has_items {
                    let selected_idx = selected_index.get();
                    let pos = dropdown_position.get();
                    let dropdown_class = if pos == "top" {
                        "autocomplete-dropdown autocomplete-dropdown-top"
                    } else {
                        "autocomplete-dropdown"
                    };
                    Some(view! {
                        <div class=dropdown_class>
                            {filtered
                                .iter()
                                .enumerate()
                                .map(|(idx, c)| {
                                    let char_id = c.id;
                                    let char_name = c.name.clone();
                                    let fighter_key = c.fighter_key.clone();
                                    let is_selected = idx == selected_idx;
                                    view! {
                                        <div
                                            class=move || {
                                                if is_selected {
                                                    "autocomplete-item selected"
                                                } else {
                                                    "autocomplete-item"
                                                }
                                            }

                                            on:mousedown=move |ev| {
                                                ev.prevent_default();
                                                handle_select(char_id);
                                            }
                                        >
                                            <img
                                                src=format!("/public/fighters/{}.png", fighter_key)
                                                class="character-icon"
                                                alt=char_name.clone()
                                            />
                                            <span>{char_name}</span>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
