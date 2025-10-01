use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

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
    let (editing_char, set_editing_char) = signal(false);
    let (editing_opp, set_editing_opp) = signal(false);
    let (editing_result, set_editing_result) = signal(false);
    let (comment_value, set_comment_value) = signal(initial_comment.clone());
    let (show_menu, set_show_menu) = signal(false);
    let (dropdown_pos, set_dropdown_pos) = signal((0.0, 0.0));

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

    let save_comment = move |_should_close: bool| {
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

    let mut characters_for_char_select = characters.clone();
    characters_for_char_select.sort_by_key(|c| c.id);
    let mut characters_for_opp_select = characters.clone();
    characters_for_opp_select.sort_by_key(|c| c.id);

    let handle_delete = move || {
        set_show_menu.set(false);
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
                                {characters_for_char_select
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let char_name = c.name.clone();
                                        let fighter_key = c.fighter_key.clone();
                                        view! {
                                            <div
                                                class="char-dropdown-item"
                                                on:click=move |_| {
                                                    set_selected_char_id.set(char_id);
                                                    save_character(char_id);
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
                            let (name, key) = get_char_info(selected_char_id.get());
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
                                {characters_for_opp_select
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let char_name = c.name.clone();
                                        let fighter_key = c.fighter_key.clone();
                                        view! {
                                            <div
                                                class="char-dropdown-item"
                                                on:click=move |_| {
                                                    set_selected_opp_id.set(char_id);
                                                    save_opponent(char_id);
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
                            let (name, key) = get_opp_info(selected_opp_id.get());
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
                        }}
                    </div>
                </div>

                <input
                    type="text"
                    class="match-comment-input"
                    value=comment_value
                    on:input=move |ev| set_comment_value.set(event_target_value(&ev))
                    on:blur=move |_| save_comment(false)
                />

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
        </div>
    }
}
