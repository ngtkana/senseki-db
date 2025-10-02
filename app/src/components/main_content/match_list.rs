use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use std::collections::HashSet;

use crate::api::{self, Character, CreateMatchRequest, Match, MatchResult};
use crate::utils::match_result::get_result_button_class;

use super::character_selector::CharacterSelector;
use super::match_item::MatchItem;

#[derive(Clone, Debug)]
struct DraftMatch {
    character_id: Option<i32>,
    opponent_character_id: Option<i32>,
    result: Option<MatchResult>,
    comment: String,
}

/// 共通のマッチ行レイアウトコンポーネント
#[component]
pub(super) fn MatchRowLayout<LC, CS, RB, CI>(
    left_control: LC,
    match_number: Option<usize>,
    character_selector: CS,
    result_buttons: RB,
    comment_input: CI,
) -> impl IntoView
where
    LC: IntoView + 'static,
    CS: IntoView + 'static,
    RB: IntoView + 'static,
    CI: IntoView + 'static,
{
    view! {
        <div class="match-row">
            {left_control}
            <div class="match-number">
                {match_number.map(|n| n.to_string()).unwrap_or_default()}
            </div>
            {character_selector}
            {result_buttons}
            {comment_input}
        </div>
    }
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
            result: None,
            comment: String::new(),
        }));
    };

    let save_draft_match =
        move |char_id: i32, opp_id: i32, result: MatchResult, comment: String| {
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
                            result: None,
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
            // result が設定されている場合のみ自動確定
            if let Some(result) = draft.result {
                if let Some(char_id) = draft.character_id {
                    save_draft_match(char_id, opp_id, result, draft.comment.clone());
                    return;
                }
            }
            set_draft_match.set(Some(draft));
        }
    };

    let handle_win_click = move || {
        if let Some(mut draft) = draft_match.get() {
            draft.result = match draft.result {
                None => Some(MatchResult::Win),                    // 未確定 → 勝ち
                Some(MatchResult::Win) => Some(MatchResult::Loss), // 勝ち → 負け（トグル）
                Some(MatchResult::Loss) => Some(MatchResult::Win), // 負け → 勝ち（トグル）
            };

            // 自キャラ、相手キャラ、勝敗が全て揃っている場合は自動確定
            if let (Some(char_id), Some(opp_id), Some(result)) = (
                draft.character_id,
                draft.opponent_character_id,
                draft.result,
            ) {
                save_draft_match(char_id, opp_id, result, draft.comment.clone());
            } else {
                set_draft_match.set(Some(draft));
            }
        }
    };

    let handle_loss_click = move || {
        if let Some(mut draft) = draft_match.get() {
            draft.result = match draft.result {
                None => Some(MatchResult::Loss),                   // 未確定 → 負け
                Some(MatchResult::Win) => Some(MatchResult::Loss), // 勝ち → 負け（トグル）
                Some(MatchResult::Loss) => Some(MatchResult::Win), // 負け → 勝ち（トグル）
            };

            // 自キャラ、相手キャラ、勝敗が全て揃っている場合は自動確定
            if let (Some(char_id), Some(opp_id), Some(result)) = (
                draft.character_id,
                draft.opponent_character_id,
                draft.result,
            ) {
                save_draft_match(char_id, opp_id, result, draft.comment.clone());
            } else {
                set_draft_match.set(Some(draft));
            }
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
            if let (Some(c), Some(o), Some(r)) = (
                draft.character_id,
                draft.opponent_character_id,
                draft.result,
            ) {
                save_draft_match(c, o, r, draft.comment.clone());
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
                let all_matches = matches.get();
                let match_count = all_matches.len();

                all_matches
                    .iter()
                    .enumerate()
                    .map(|(index, m)| {
                        let match_id = m.id;
                        let is_selected = selected.contains(&match_id);

                        // 前後のマッチのコメント入力欄にフォーカスを移すコールバック
                        let focus_prev = if index > 0 {
                            Some(Box::new(move || {
                                // 前のマッチのコメント入力欄を探してフォーカス
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        let selector = format!(".match-item:nth-child({}) .match-comment-input", index);
                                        if let Ok(Some(element)) = document.query_selector(&selector) {
                                            if let Some(input) = element.dyn_ref::<web_sys::HtmlInputElement>() {
                                                let _ = input.focus();
                                            }
                                        }
                                    }
                                }
                            }) as Box<dyn Fn() + 'static>)
                        } else {
                            None
                        };

                        let focus_next = if index < match_count - 1 {
                            Some(Box::new(move || {
                                // 次のマッチのコメント入力欄を探してフォーカス
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        let selector = format!(".match-item:nth-child({}) .match-comment-input", index + 2);
                                        if let Ok(Some(element)) = document.query_selector(&selector) {
                                            if let Some(input) = element.dyn_ref::<web_sys::HtmlInputElement>() {
                                                let _ = input.focus();
                                            }
                                        }
                                    }
                                }
                            }) as Box<dyn Fn() + 'static>)
                        } else {
                            None
                        };

                        if let (Some(_), Some(_)) = (&focus_prev, &focus_next) {
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
                                    on_focus_prev=focus_prev.unwrap()
                                    on_focus_next=focus_next.unwrap()
                                />
                            }.into_any()
                        } else if let Some(_) = &focus_prev {
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
                                    on_focus_prev=focus_prev.unwrap()
                                />
                            }.into_any()
                        } else if let Some(_) = &focus_next {
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
                                    on_focus_next=focus_next.unwrap()
                                />
                            }.into_any()
                        } else {
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
                            }.into_any()
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
                            on_win_click=handle_win_click
                            on_loss_click=handle_loss_click
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
    on_win_click: impl Fn() + 'static + Copy + Send + Sync,
    on_loss_click: impl Fn() + 'static + Copy + Send + Sync,
    on_comment_change: impl Fn(String) + 'static + Copy + Send + Sync,
    on_confirm: impl Fn() + 'static + Copy + Send + Sync,
    on_cancel: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (draft_char_id, _) = signal(draft.character_id);
    let (draft_opp_id, _) = signal(draft.opponent_character_id);
    let draft_result = draft.result.clone();
    let draft_result_2 = draft.result.clone();
    let draft_result_3 = draft.result.clone();
    let draft_result_4 = draft.result.clone();
    let draft_comment = draft.comment.clone();
    let draft_comment_2 = draft.comment.clone();
    let draft_opp_id_for_check = draft.opponent_character_id;

    let result_buttons_ref = NodeRef::<leptos::html::Div>::new();

    let characters_for_char = characters.clone();
    let characters_for_opp = characters.clone();

    // 相手キャラが選択されたら result-buttons にフォーカス
    let handle_opponent_close = move || {
        if draft_opp_id.get().is_some() {
            if let Some(element) = result_buttons_ref.get() {
                let _ = element.focus();
            }
        }
    };

    let left_control = view! {
        <button
            class="draft-delete-btn"
            on:click=move |_| {
                on_cancel();
            }
            title="ドラフトを削除"
        >
            "×"
        </button>
    }
    .into_view();

    let character_selector = view! {
        <div class="match-characters">
            <CharacterSelector
                characters=characters_for_char
                selected_id=draft_char_id
                on_select=on_character_select
                placeholder="自キャラ"
                show_icon=false
            />

            <span class="vs-text">" vs "</span>

            <CharacterSelector
                characters=characters_for_opp
                selected_id=draft_opp_id
                on_select=on_opponent_select
                placeholder="相手"
                show_icon=false
                auto_open=draft_opp_id_for_check.is_none()
                on_close=Box::new(handle_opponent_close)
            />
        </div>
    }
    .into_view();

    let result_buttons = view! {
        <div
            class="result-buttons"
            tabindex="0"
            node_ref=result_buttons_ref
            on:keydown=move |ev| {
                let key = ev.key();
                match key.as_str() {
                    "w" | "W" => {
                        ev.prevent_default();
                        on_win_click();
                    }
                    "l" | "L" => {
                        ev.prevent_default();
                        on_loss_click();
                    }
                    "Enter" => {
                        ev.prevent_default();
                        // 相手キャラと勝敗が入力済みの場合のみ確定
                        if draft_opp_id.get().is_some() && draft_result_4.is_some() {
                            on_confirm();
                        }
                    }
                    _ => {}
                }
            }
        >
            <button
                class=move || get_result_button_class(draft_result.clone(), MatchResult::Win)
                on:click=move |ev| {
                    ev.stop_propagation();
                    on_win_click();
                }
            >
                "○"
            </button>
            <button
                class=move || get_result_button_class(draft_result_2.clone(), MatchResult::Loss)
                on:click=move |ev| {
                    ev.stop_propagation();
                    on_loss_click();
                }
            >
                "×"
            </button>
        </div>
    }
    .into_view();

    let comment_input = view! {
        <input
            type="text"
            class="match-comment-input"
            value=draft_comment
            on:input=move |ev| {
                on_comment_change(event_target_value(&ev));
            }
        />
    }
    .into_view();

    view! {
        <div
            class="match-item draft-match"
            on:keydown=move |ev| {
                // Esc キーでドラフトをキャンセル（未入力の場合のみ）
                if ev.key() == "Escape" {
                    if draft_opp_id.get().is_none()
                        && draft_result_3.is_none()
                        && draft_comment_2.is_empty()
                    {
                        ev.prevent_default();
                        on_cancel();
                    }
                }
            }
        >
            <MatchRowLayout
                left_control=left_control
                match_number=None
                character_selector=character_selector
                result_buttons=result_buttons
                comment_input=comment_input
            />
        </div>
    }
}
