use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::collections::HashSet;

use crate::api::{self, Character, CreateMatchRequest, Match};

use super::character_selector::CharacterSelector;
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

    let flip_draft_result = move || {
        if let Some(mut draft) = draft_match.get() {
            // flip: 勝ち→負け、負け→勝ち、空→勝ち
            draft.result = match draft.result.as_str() {
                "win" => "loss".to_string(),
                "loss" => "win".to_string(),
                _ => "win".to_string(),
            };
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
                            on_result_change=flip_draft_result
                            on_comment_change=update_draft_comment
                            _on_confirm=confirm_draft
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
    on_result_change: impl Fn() + 'static + Copy + Send + Sync,
    on_comment_change: impl Fn(String) + 'static + Copy + Send + Sync,
    _on_confirm: impl Fn() + 'static + Copy + Send + Sync,
    on_cancel: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (draft_char_id, _) = signal(draft.character_id);
    let (draft_opp_id, _) = signal(draft.opponent_character_id);
    let draft_result = draft.result.clone();
    let draft_result_2 = draft.result.clone();
    let draft_comment = draft.comment.clone();

    let characters_for_char = characters.clone();
    let characters_for_opp = characters.clone();

    view! {
        <div class="match-item draft-match">
            <div class="match-row">
                <button
                    class="draft-delete-btn"
                    on:click=move |_| {
                        on_cancel();
                    }
                    title="ドラフトを削除"
                >
                    "×"
                </button>
                <div class="match-number"></div>
                <div class="match-characters">
                    <CharacterSelector
                        characters=characters_for_char
                        selected_id=draft_char_id.into()
                        on_select=on_character_select
                        placeholder="自キャラ"
                        show_icon=false
                    />

                    <span class="vs-text">" vs "</span>

                    <CharacterSelector
                        characters=characters_for_opp
                        selected_id=draft_opp_id.into()
                        on_select=on_opponent_select
                        placeholder="相手"
                        show_icon=false
                    />
                </div>

                <input
                    type="text"
                    class="match-comment-input"
                    value=draft_comment
                    on:input=move |ev| {
                        on_comment_change(event_target_value(&ev));
                    }
                />

                <div class="result-buttons" on:click=move |_| on_result_change()>
                    <button
                        class=move || {
                            let result = draft_result.clone();
                            if result == "win" {
                                "result-btn result-btn-win active"
                            } else if result.is_empty() {
                                "result-btn result-btn-win unselected"
                            } else {
                                "result-btn result-btn-win inactive"
                            }
                        }
                    >
                        "○"
                    </button>
                    <button
                        class=move || {
                            let result = draft_result_2.clone();
                            if result == "loss" {
                                "result-btn result-btn-loss active"
                            } else if result.is_empty() {
                                "result-btn result-btn-loss unselected"
                            } else {
                                "result-btn result-btn-loss inactive"
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
