use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{self, Character, Match, UpdateMatchRequest};
use crate::utils::match_result::get_result_button_class;

use super::character_selector::CharacterSelector;
use super::match_list::MatchRowLayout;

#[component]
pub fn MatchItem(
    match_number: usize,
    match_data: Match,
    char_name: String,
    opp_name: String,
    characters: Vec<Character>,
    is_selected: bool,
    on_match_clicked: impl Fn(bool, bool) + 'static + Copy + Send + Sync,
    _on_match_deleted: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let initial_comment = match_data.comment.clone().unwrap_or_default();
    let (comment_value, set_comment_value) = signal(initial_comment.clone());

    let char_id = characters
        .iter()
        .find(|c| c.name == char_name)
        .map(|c| c.id);
    let opp_id = characters.iter().find(|c| c.name == opp_name).map(|c| c.id);

    let (selected_char_id, set_selected_char_id) = signal(char_id);
    let (selected_opp_id, set_selected_opp_id) = signal(opp_id);
    let (result_value, set_result_value) = signal(match_data.result.clone());

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
        set_selected_char_id.set(Some(new_char_id));
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
                }
                Err(e) => {
                    logging::error!("自キャラ更新失敗: {}", e);
                }
            }
        });
    };

    let save_opponent = move |new_opp_id: i32| {
        set_selected_opp_id.set(Some(new_opp_id));
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
                }
                Err(e) => {
                    logging::error!("勝敗更新失敗: {}", e);
                }
            }
        });
    };

    let characters_for_char = characters.clone();
    let characters_for_opp = characters.clone();

    let left_control = view! {
        <input
            type="checkbox"
            class="match-checkbox"
            checked=is_selected
            on:click=move |ev| {
                ev.stop_propagation();
                let shift_key = ev.shift_key();
                let ctrl_key = ev.ctrl_key() || ev.meta_key();
                on_match_clicked(shift_key, ctrl_key);
            }
        />
    }
    .into_view();

    let character_selector = view! {
        <div class="match-characters">
            <CharacterSelector
                characters=characters_for_char
                selected_id=selected_char_id
                on_select=save_character
                show_icon=false
            />

            <span class="vs-text">" vs "</span>

            <CharacterSelector
                characters=characters_for_opp
                selected_id=selected_opp_id
                on_select=save_opponent
                show_icon=false
            />
        </div>
    }
    .into_view();

    let result_buttons = view! {
        <div
            class="result-buttons"
            on:click=move |_| {
                let current = result_value.get();
                let new_result = if current == "win" { "loss" } else { "win" };
                save_result(new_result.to_string());
            }
        >
            <button class=move || get_result_button_class(&result_value.get(), "win")>
                "○"
            </button>
            <button class=move || get_result_button_class(&result_value.get(), "loss")>
                "×"
            </button>
        </div>
    }
    .into_view();

    let comment_input = view! {
        <input
            type="text"
            class="match-comment-input"
            value=comment_value
            on:input=move |ev| set_comment_value.set(event_target_value(&ev))
            on:blur=move |_| save_comment(false)
        />
    }
    .into_view();

    view! {
        <div class=move || if is_selected { "match-item selected" } else { "match-item" }>
            <MatchRowLayout
                left_control=left_control
                match_number=Some(match_number)
                character_selector=character_selector
                result_buttons=result_buttons
                comment_input=comment_input
            />
        </div>
    }
}
