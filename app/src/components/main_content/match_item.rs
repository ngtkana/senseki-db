use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{self, Character, Match, UpdateMatchRequest};

#[component]
pub fn MatchItem(
    match_data: Match,
    char_name: String,
    opp_name: String,
    characters: Vec<Character>,
    on_match_deleted: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let initial_comment = match_data.comment.clone().unwrap_or_default();
    let (editing_comment, set_editing_comment) = signal(false);
    let (editing_char, set_editing_char) = signal(false);
    let (editing_opp, set_editing_opp) = signal(false);
    let (editing_result, set_editing_result) = signal(false);
    let (comment_value, set_comment_value) = signal(initial_comment.clone());
    let (show_menu, set_show_menu) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);

    let char_id = characters
        .iter()
        .find(|c| c.name == char_name)
        .map(|c| c.id)
        .unwrap_or(0);
    let opp_id = characters
        .iter()
        .find(|c| c.name == opp_name)
        .map(|c| c.id)
        .unwrap_or(0);

    let (selected_char_id, set_selected_char_id) = signal(char_id);
    let (selected_opp_id, set_selected_opp_id) = signal(opp_id);
    let (result_value, set_result_value) = signal(match_data.result.clone());

    let result_class = move || {
        if result_value.get() == "win" {
            "win"
        } else {
            "loss"
        }
    };
    let result_symbol = move || {
        if result_value.get() == "win" {
            "○"
        } else {
            "×"
        }
    };

    let match_id = match_data.id;

    let save_comment = move |should_close: bool| {
        let new_comment = comment_value.get();
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: None,
                opponent_character_id: None,
                result: None,
                comment: Some(new_comment),
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("コメント更新成功");
                    if should_close {
                        set_editing_comment.set(false);
                    }
                }
                Err(e) => {
                    logging::error!("コメント更新失敗: {}", e);
                }
            }
        });
    };

    let save_character = move |new_char_id: i32| {
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: Some(new_char_id),
                opponent_character_id: None,
                result: None,
                comment: None,
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("自キャラ更新成功");
                    set_editing_char.set(false);
                }
                Err(e) => {
                    logging::error!("自キャラ更新失敗: {}", e);
                }
            }
        });
    };

    let save_opponent = move |new_opp_id: i32| {
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: None,
                opponent_character_id: Some(new_opp_id),
                result: None,
                comment: None,
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("相手キャラ更新成功");
                    set_editing_opp.set(false);
                }
                Err(e) => {
                    logging::error!("相手キャラ更新失敗: {}", e);
                }
            }
        });
    };

    let save_result = move |new_result: String| {
        spawn_local(async move {
            let req = UpdateMatchRequest {
                character_id: None,
                opponent_character_id: None,
                result: Some(new_result.clone()),
                comment: None,
            };
            match api::update_match(match_id, req).await {
                Ok(_) => {
                    logging::log!("勝敗更新成功");
                    set_result_value.set(new_result);
                    set_editing_result.set(false);
                }
                Err(e) => {
                    logging::error!("勝敗更新失敗: {}", e);
                }
            }
        });
    };

    let characters_for_char_info = characters.clone();
    let get_char_info = move |id: i32| {
        characters_for_char_info
            .iter()
            .find(|c| c.id == id)
            .map(|c| (c.name.clone(), c.fighter_key.clone()))
            .unwrap_or_default()
    };

    let characters_for_opp_info = characters.clone();
    let get_opp_info = move |id: i32| {
        characters_for_opp_info
            .iter()
            .find(|c| c.id == id)
            .map(|c| (c.name.clone(), c.fighter_key.clone()))
            .unwrap_or_default()
    };

    let characters_for_char_select = characters.clone();
    let characters_for_opp_select = characters.clone();

    let handle_delete = move || {
        set_show_delete_confirm.set(true);
        set_show_menu.set(false);
    };

    let confirm_delete = move || {
        spawn_local(async move {
            match api::delete_match(match_id).await {
                Ok(_) => {
                    logging::log!("マッチ削除成功");
                    on_match_deleted();
                }
                Err(e) => {
                    logging::error!("マッチ削除失敗: {}", e);
                }
            }
        });
    };

    view! {
        <div class="match-item">
            <div class="match-row">
                <div class="match-characters">
                    <Show
                        when=move || editing_char.get()
                        fallback=move || {
                            let (name, key) = get_char_info(selected_char_id.get());
                            view! {
                                <div
                                    class="char-display editable"
                                    on:click=move |_| set_editing_char.set(true)
                                >
                                    <img
                                        src=format!("/public/fighters/{}.png", key)
                                        class="character-icon"
                                        alt=name.clone()
                                    />
                                    <span>{name}</span>
                                </div>
                            }
                        }
                    >

                        <select
                            class="char-select"
                            on:change=move |ev| {
                                let new_id = event_target_value(&ev).parse().unwrap_or(0);
                                set_selected_char_id.set(new_id);
                                save_character(new_id);
                            }

                            on:blur=move |_| set_editing_char.set(false)
                            autofocus
                        >
                            {characters_for_char_select
                                .iter()
                                .map(|c| {
                                    let char_id = c.id;
                                    let is_selected = selected_char_id.get() == char_id;
                                    view! {
                                        <option value=char_id selected=is_selected>
                                            {c.name.clone()}
                                        </option>
                                    }
                                })
                                .collect_view()}

                        </select>
                    </Show>

                    <span class="vs-text">" vs "</span>

                    <Show
                        when=move || editing_opp.get()
                        fallback=move || {
                            let (name, key) = get_opp_info(selected_opp_id.get());
                            view! {
                                <div
                                    class="char-display editable"
                                    on:click=move |_| set_editing_opp.set(true)
                                >
                                    <img
                                        src=format!("/public/fighters/{}.png", key)
                                        class="character-icon"
                                        alt=name.clone()
                                    />
                                    <span>{name}</span>
                                </div>
                            }
                        }
                    >

                        <select
                            class="char-select"
                            on:change=move |ev| {
                                let new_id = event_target_value(&ev).parse().unwrap_or(0);
                                set_selected_opp_id.set(new_id);
                                save_opponent(new_id);
                            }

                            on:blur=move |_| set_editing_opp.set(false)
                            autofocus
                        >
                            {characters_for_opp_select
                                .iter()
                                .map(|c| {
                                    let char_id = c.id;
                                    let is_selected = selected_opp_id.get() == char_id;
                                    view! {
                                        <option value=char_id selected=is_selected>
                                            {c.name.clone()}
                                        </option>
                                    }
                                })
                                .collect_view()}

                        </select>
                    </Show>
                </div>

                <Show
                    when=move || editing_comment.get()
                    fallback=move || {
                        view! {
                            <div
                                class="match-comment editable"
                                on:click=move |_| set_editing_comment.set(true)
                            >
                                {move || {
                                    let c = comment_value.get();
                                    if c.is_empty() { "コメントを追加...".to_string() } else { c }
                                }}
                            </div>
                        }
                    }
                >
                    <input
                        type="text"
                        class="match-comment-input"
                        value=comment_value
                        on:input=move |ev| set_comment_value.set(event_target_value(&ev))
                        on:blur=move |_| {
                            save_comment(true);
                        }
                        on:keydown=move |ev: leptos::web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                save_comment(true);
                            } else if ev.key() == "Escape" {
                                set_editing_comment.set(false);
                            }
                        }
                        autofocus
                    />
                </Show>

                <Show
                    when=move || editing_result.get()
                    fallback=move || {
                        view! {
                            <div
                                class=move || format!("match-result {} editable", result_class())
                                on:click=move |_| set_editing_result.set(true)
                            >
                                {result_symbol}
                            </div>
                        }
                    }
                >

                    <select
                        class="result-select"
                        on:change=move |ev| {
                            let new_result = event_target_value(&ev);
                            save_result(new_result);
                        }

                        on:blur=move |_| set_editing_result.set(false)
                        autofocus
                    >
                        <option value="win" selected=move || result_value.get() == "win">
                            "勝ち"
                        </option>
                        <option value="loss" selected=move || result_value.get() == "loss">
                            "負け"
                        </option>
                    </select>
                </Show>

                <div class="match-menu">
                    <button
                        class="menu-button"
                        on:click=move |_| set_show_menu.set(!show_menu.get())
                    >
                        "⋮"
                    </button>
                    <Show when=move || show_menu.get()>
                        <div class="menu-dropdown">
                            <button class="menu-item delete" on:click=move |_| handle_delete()>
                                "削除"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>

            <Show when=move || show_delete_confirm.get()>
                <div class="confirm-overlay" on:click=move |_| set_show_delete_confirm.set(false)>
                    <div class="confirm-dialog" on:click=|e| e.stop_propagation()>
                        <h3>"マッチを削除しますか？"</h3>
                        <p>"この操作は取り消せません。"</p>
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
        </div>
    }
}
