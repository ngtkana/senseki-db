use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

use crate::api::{self, Character, CreateMatchRequest, Match};

use super::match_item::MatchItem;

#[derive(Clone, Debug)]
struct DraftMatch {
    character_id: Option<i32>,
    opponent_character_id: Option<i32>,
    result: String,
}

#[component]
pub fn MatchList(
    session_id: i32,
    matches: ReadSignal<Vec<Match>>,
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_match_added: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (draft_match, set_draft_match) = signal(Some(DraftMatch {
        character_id: selected_character_id.get(),
        opponent_character_id: None,
        result: "win".to_string(),
    }));

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

    let save_draft_match = move |char_id: i32, opp_id: i32, result: String| {
        spawn_local(async move {
            let req = CreateMatchRequest {
                session_id,
                character_id: char_id,
                opponent_character_id: opp_id,
                result,
                comment: None,
            };
            match api::create_match(req).await {
                Ok(_) => {
                    logging::log!("マッチ追加成功");
                    // 新しいドラフトを作成
                    set_draft_match.set(Some(DraftMatch {
                        character_id: selected_character_id.get(),
                        opponent_character_id: None,
                        result: "win".to_string(),
                    }));
                    on_match_added();
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
            // 両方選択されたら保存
            if let (Some(c), Some(o)) = (draft.character_id, draft.opponent_character_id) {
                save_draft_match(c, o, draft.result.clone());
            } else {
                set_draft_match.set(Some(draft));
            }
        }
    };

    let update_draft_opponent = move |opp_id: i32| {
        if let Some(mut draft) = draft_match.get() {
            draft.opponent_character_id = Some(opp_id);
            // 両方選択されたら保存
            if let (Some(c), Some(o)) = (draft.character_id, draft.opponent_character_id) {
                save_draft_match(c, o, draft.result.clone());
            } else {
                set_draft_match.set(Some(draft));
            }
        }
    };

    let update_draft_result = move |result: String| {
        if let Some(mut draft) = draft_match.get() {
            draft.result = result.clone();
            // 両方選択されたら保存
            if let (Some(c), Some(o)) = (draft.character_id, draft.opponent_character_id) {
                save_draft_match(c, o, result);
            } else {
                set_draft_match.set(Some(draft));
            }
        }
    };

    view! {
        <div class="match-list">
            {move || {
                let chars = characters.get();
                matches
                    .get()
                    .iter()
                    .map(|m| {
                        view! {
                            <MatchItem
                                match_data=m.clone()
                                char_name=m.character_name.clone()
                                opp_name=m.opponent_character_name.clone()
                                characters=chars.clone()
                                on_match_deleted=on_match_added
                            />
                        }
                    })
                    .collect_view()
            }}

            {move || {
                let draft = draft_match.get().unwrap();
                let chars = characters.get();
                view! {
                    <DraftMatchItem
                        draft=draft
                        characters=chars
                        on_character_select=update_draft_character
                        on_opponent_select=update_draft_opponent
                        on_result_change=update_draft_result
                    />
                }
            }}
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

    view! {
        <div class="match-item draft-match">
            <div class="match-row">
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
                                    set_dropdown_pos.set((rect.left(), rect.bottom()));
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
                                    set_dropdown_pos.set((rect.left(), rect.bottom()));
                                }
                            }
                            set_editing_opp.set(true);
                        }
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
                    placeholder="コメント（任意）"
                    disabled
                />

                <select
                    class="result-select"
                    on:change=move |ev| {
                        let new_result = event_target_value(&ev);
                        on_result_change(new_result);
                    }
                >
                    <option value="win" selected=move || draft_result.clone() == "win">
                        "勝ち"
                    </option>
                    <option value="loss" selected=move || draft_result_2.clone() == "loss">
                        "負け"
                    </option>
                </select>
            </div>
        </div>
    }
}
