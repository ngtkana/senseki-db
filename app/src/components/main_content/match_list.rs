use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use std::collections::HashSet;

use crate::api::{self, Character, CreateMatchRequest, Match};

use super::match_item::MatchItem;

#[derive(Clone, Debug)]
struct DraftMatch {
    character_id: Option<i32>,
    opponent_character_id: Option<i32>,
    result: String,
    comment: String,
}

#[component]
pub fn MatchList(
    session_id: i32,
    matches: ReadSignal<Vec<Match>>,
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_match_added: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (draft_match, set_draft_match) = signal(Option::<DraftMatch>::None);
    let (selected_matches, set_selected_matches) = signal(HashSet::<i32>::new());
    let (last_selected_index, set_last_selected_index) = signal(Option::<usize>::None);

    // グローバルな自キャラが変更されたら、ドラフトの自キャラも更新
    Effect::new(move || {
        let selected_char = selected_character_id.get();
        if let Some(mut draft) = draft_match.get() {
            if draft.character_id != selected_char {
                draft.character_id = selected_char;
                set_draft_match.set(Some(draft));
            }
        }
    });

    let add_draft_match = move || {
        set_draft_match.set(Some(DraftMatch {
            character_id: selected_character_id.get(),
            opponent_character_id: None,
            result: "win".to_string(),
            comment: String::new(),
        }));
    };

    let save_draft_match = move |char_id: i32, opp_id: i32, result: String, comment: String| {
        spawn_local(async move {
            let req = CreateMatchRequest {
                session_id,
                character_id: char_id,
                opponent_character_id: opp_id,
                result,
                comment: if comment.is_empty() {
                    None
                } else {
                    Some(comment)
                },
            };
            match api::create_match(req).await {
                Ok(_) => {
                    logging::log!("マッチ追加成功");
                    on_match_added();
                    // 次のドラフトを自動作成
                    set_draft_match.set(Some(DraftMatch {
                        character_id: selected_character_id.get(),
                        opponent_character_id: None,
                        result: "win".to_string(),
                        comment: String::new(),
                    }));
                }
                Err(e) => {
                    logging::error!("マッチ追加失敗: {}", e);
                }
            }
        });
    };

    let update_draft_character = move |char_id: i32| {
        if let Some(mut draft) = draft_match.get() {
            draft.character_id = Some(char_id);
            set_draft_match.set(Some(draft));
        }
    };

    let update_draft_opponent = move |opp_id: i32| {
        if let Some(mut draft) = draft_match.get() {
            draft.opponent_character_id = Some(opp_id);
            // 相手キャラが選択されたら自動的に確定
            if let Some(char_id) = draft.character_id {
                save_draft_match(char_id, opp_id, draft.result.clone(), draft.comment.clone());
            }
        }
    };

    let update_draft_result = move |result: String| {
        if let Some(mut draft) = draft_match.get() {
            draft.result = result;
            set_draft_match.set(Some(draft));
        }
    };

    let update_draft_comment = move |comment: String| {
        if let Some(mut draft) = draft_match.get() {
            draft.comment = comment;
            set_draft_match.set(Some(draft));
        }
    };

    let confirm_draft = move || {
        if let Some(draft) = draft_match.get() {
            if let (Some(c), Some(o)) = (draft.character_id, draft.opponent_character_id) {
                save_draft_match(c, o, draft.result.clone(), draft.comment.clone());
            }
        }
    };

    let handle_match_click = move |match_id: i32, index: usize, shift_key: bool, ctrl_key: bool| {
        let mut selected = selected_matches.get();

        if shift_key {
            // Shift選択: 範囲選択
            if let Some(last_idx) = last_selected_index.get() {
                let start = last_idx.min(index);
                let end = last_idx.max(index);
                let all_matches = matches.get();
                for i in start..=end {
                    if i < all_matches.len() {
                        selected.insert(all_matches[i].id);
                    }
                }
            } else {
                selected.insert(match_id);
            }
            set_last_selected_index.set(Some(index));
        } else if ctrl_key {
            // Ctrl選択: 複数選択
            if selected.contains(&match_id) {
                selected.remove(&match_id);
            } else {
                selected.insert(match_id);
            }
            set_last_selected_index.set(Some(index));
        } else {
            // 通常クリック: トグル選択
            if selected.contains(&match_id) {
                selected.remove(&match_id);
            } else {
                selected.insert(match_id);
            }
            set_last_selected_index.set(Some(index));
        }

        set_selected_matches.set(selected);
    };

    let delete_selected = move || {
        let selected = selected_matches.get();
        if selected.is_empty() {
            return;
        }

        for match_id in selected.iter() {
            let id = *match_id;
            spawn_local(async move {
                match api::delete_match(id).await {
                    Ok(_) => {
                        logging::log!("マッチ削除成功: {}", id);
                    }
                    Err(e) => {
                        logging::error!("マッチ削除失敗: {}", e);
                    }
                }
            });
        }

        set_selected_matches.set(HashSet::new());
        set_last_selected_index.set(None);
        on_match_added();
    };

    view! {
        <div class="match-list">
            <Show when=move || !selected_matches.get().is_empty()>
                <div class="match-list-header">
                    <span class="selected-count">
                        {move || format!("{}件選択中", selected_matches.get().len())}
                    </span>
                    <button class="btn btn-danger" on:click=move |_| delete_selected()>
                        "選択を削除"
                    </button>
                </div>
            </Show>
            {move || {
                let chars = characters.get();
                let selected = selected_matches.get();
                matches
                    .get()
                    .iter()
                    .enumerate()
                    .map(|(index, m)| {
                        let match_id = m.id;
                        let is_selected = selected.contains(&match_id);
                        view! {
                            <MatchItem
                                match_number=index + 1
                                match_data=m.clone()
                                char_name=m.character_name.clone()
                                opp_name=m.opponent_character_name.clone()
                                characters=chars.clone()
                                is_selected=is_selected
                                on_match_clicked=move |shift, ctrl| handle_match_click(match_id, index, shift, ctrl)
                                _on_match_deleted=on_match_added
                            />
                        }
                    })
                    .collect_view()
            }}

            <Show when=move || draft_match.get().is_some()>
                {move || {
                    let draft = draft_match.get().unwrap();
                    let chars = characters.get();
                    let cancel_draft = move || set_draft_match.set(None);
                    view! {
                        <DraftMatchItem
                            draft=draft
                            characters=chars
                            on_character_select=update_draft_character
                            on_opponent_select=update_draft_opponent
                            on_result_change=update_draft_result
                            on_comment_change=update_draft_comment
                            on_confirm=confirm_draft
                            on_cancel=cancel_draft
                        />
                    }
                }}
            </Show>

            <Show when=move || draft_match.get().is_none()>
                <button class="add-match-button" on:click=move |_| add_draft_match()>
                    "+ マッチを追加"
                </button>
            </Show>
        </div>
    }
}

#[component]
fn DraftMatchItem(
    draft: DraftMatch,
    characters: Vec<Character>,
    on_character_select: impl Fn(i32) + 'static + Copy + Send + Sync,
    on_opponent_select: impl Fn(i32) + 'static + Copy + Send + Sync,
    on_result_change: impl Fn(String) + 'static + Copy + Send + Sync,
    on_comment_change: impl Fn(String) + 'static + Copy + Send + Sync,
    on_confirm: impl Fn() + 'static + Copy + Send + Sync,
    on_cancel: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (editing_char, set_editing_char) = signal(draft.character_id.is_none());
    let (editing_opp, set_editing_opp) = signal(false);
    let (dropdown_pos, set_dropdown_pos) = signal((0.0, 0.0));

    let characters_for_dropdown = characters.clone();
    let mut characters_sorted = characters_for_dropdown.clone();
    characters_sorted.sort_by_key(|c| c.id);
    let characters_sorted_2 = characters_sorted.clone();

    let characters_for_char_info = characters.clone();
    let get_char_info = move |id: Option<i32>| {
        id.and_then(|id| {
            characters_for_char_info
                .iter()
                .find(|c| c.id == id)
                .map(|c| (c.name.clone(), c.fighter_key.clone()))
        })
    };

    let characters_for_opp_info = characters.clone();
    let get_opp_info = move |id: Option<i32>| {
        id.and_then(|id| {
            characters_for_opp_info
                .iter()
                .find(|c| c.id == id)
                .map(|c| (c.name.clone(), c.fighter_key.clone()))
        })
    };

    let draft_char_id = draft.character_id;
    let draft_opp_id = draft.opponent_character_id;
    let draft_result = draft.result.clone();
    let draft_result_2 = draft.result.clone();
    let draft_result_3 = draft.result.clone();
    let draft_result_4 = draft.result.clone();
    let draft_comment = draft.comment.clone();

    let opp_ref = NodeRef::<leptos::html::Div>::new();

    // 自キャラが選択されたら相手キャラ選択にフォーカス
    Effect::new(move || {
        if draft_char_id.is_some() && draft_opp_id.is_none() {
            if let Some(div_elem) = opp_ref.get() {
                let _ = div_elem.click();
            }
        }
    });

    view! {
        <div class="match-item draft-match">
            <div class="match-row">
                <input
                    type="checkbox"
                    class="match-checkbox"
                    checked=false
                    on:click=move |ev| {
                        ev.stop_propagation();
                        on_cancel();
                    }
                />
                <div class="match-number"></div>
                <div class="match-characters">
                    <Show when=move || editing_char.get()>
                        <div class="char-dropdown-overlay" on:click=move |_| set_editing_char.set(false)>
                            <div
                                class="char-dropdown"
                                style=move || {
                                    let (left, top) = dropdown_pos.get();
                                    format!("left: {}px; top: {}px;", left, top)
                                }
                                on:click=|e| e.stop_propagation()
                            >
                                {characters_sorted
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let char_name = c.name.clone();
                                        let fighter_key = c.fighter_key.clone();
                                        view! {
                                            <div
                                                class="char-dropdown-item"
                                                on:click=move |_| {
                                                    on_character_select(char_id);
                                                    set_editing_char.set(false);
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
                        </div>
                    </Show>

                    <div
                        class="char-display editable"
                        on:click=move |ev| {
                            if let Some(element) = ev.current_target() {
                                if let Some(el) = element.dyn_ref::<web_sys::HtmlElement>() {
                                    let elem = el.as_ref() as &web_sys::Element;
                                    let rect = elem.get_bounding_client_rect();
                                    let window_height = web_sys::window()
                                        .and_then(|w| w.inner_height().ok())
                                        .and_then(|h| h.as_f64())
                                        .unwrap_or(600.0);

                                    let dropdown_height = 400.0;
                                    let space_below = window_height - rect.bottom();

                                    let top = if space_below < dropdown_height {
                                        rect.top() - dropdown_height
                                    } else {
                                        rect.bottom()
                                    };

                                    set_dropdown_pos.set((rect.left(), top));
                                }
                            }
                            set_editing_char.set(true);
                        }
                    >
                        {move || {
                            if let Some((name, key)) = get_char_info(draft_char_id) {
                                view! {
                                    <>
                                        <img
                                            src=format!("/public/fighters/{}.png", key)
                                            class="character-icon"
                                            alt=name.clone()
                                        />
                                        <span>{name}</span>
                                    </>
                                }
                                    .into_any()
                            } else {
                                view! { <span style="color: #999;">"キャラを選択"</span> }.into_any()
                            }
                        }}
                    </div>

                    <span class="vs-text">" vs "</span>

                    <Show when=move || editing_opp.get()>
                        <div class="char-dropdown-overlay" on:click=move |_| set_editing_opp.set(false)>
                            <div
                                class="char-dropdown"
                                style=move || {
                                    let (left, top) = dropdown_pos.get();
                                    format!("left: {}px; top: {}px;", left, top)
                                }
                                on:click=|e| e.stop_propagation()
                            >
                                {characters_sorted_2
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let char_name = c.name.clone();
                                        let fighter_key = c.fighter_key.clone();
                                        view! {
                                            <div
                                                class="char-dropdown-item"
                                                on:click=move |_| {
                                                    on_opponent_select(char_id);
                                                    set_editing_opp.set(false);
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
                        </div>
                    </Show>

                    <div
                        class="char-display editable"
                        on:click=move |ev| {
                            if let Some(element) = ev.current_target() {
                                if let Some(el) = element.dyn_ref::<web_sys::HtmlElement>() {
                                    let elem = el.as_ref() as &web_sys::Element;
                                    let rect = elem.get_bounding_client_rect();
                                    let window_height = web_sys::window()
                                        .and_then(|w| w.inner_height().ok())
                                        .and_then(|h| h.as_f64())
                                        .unwrap_or(600.0);

                                    let dropdown_height = 400.0;
                                    let space_below = window_height - rect.bottom();

                                    let top = if space_below < dropdown_height {
                                        rect.top() - dropdown_height
                                    } else {
                                        rect.bottom()
                                    };

                                    set_dropdown_pos.set((rect.left(), top));
                                }
                            }
                            set_editing_opp.set(true);
                        }
                        node_ref=opp_ref
                    >
                        {move || {
                            if let Some((name, key)) = get_opp_info(draft_opp_id) {
                                view! {
                                    <>
                                        <img
                                            src=format!("/public/fighters/{}.png", key)
                                            class="character-icon"
                                            alt=name.clone()
                                        />
                                        <span>{name}</span>
                                    </>
                                }
                                    .into_any()
                            } else {
                                view! { <span style="color: #999;">"相手を選択"</span> }.into_any()
                            }
                        }}
                    </div>
                </div>

                <input
                    type="text"
                    class="match-comment-input"
                    value=draft_comment
                    on:input=move |ev| {
                        on_comment_change(event_target_value(&ev));
                    }
                />

                <div class="result-buttons">
                    <button
                        class=move || {
                            if draft_result.clone() == "win" {
                                "result-btn result-btn-win active"
                            } else {
                                "result-btn result-btn-win"
                            }
                        }

                        on:click=move |_| {
                            if draft_result_3.clone() != "win" {
                                on_result_change("win".to_string());
                            }
                        }
                    >
                        "○"
                    </button>
                    <button
                        class=move || {
                            if draft_result_2.clone() == "loss" {
                                "result-btn result-btn-loss active"
                            } else {
                                "result-btn result-btn-loss"
                            }
                        }

                        on:click=move |_| {
                            if draft_result_4.clone() != "loss" {
                                on_result_change("loss".to_string());
                            }
                        }
                    >
                        "×"
                    </button>
                </div>
            </div>
        </div>
    }
}
