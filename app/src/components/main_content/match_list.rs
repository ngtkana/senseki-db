use leptos::prelude::*;

use crate::api::{Character, Match};

use super::match_form::InlineMatchForm;
use super::match_item::MatchItem;

#[component]
pub fn MatchList(
    session_id: i32,
    matches: ReadSignal<Vec<Match>>,
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_match_added: impl Fn() + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (adding, set_adding) = signal(false);

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

            <Show
                when=move || !adding.get()
                fallback=move || {
                    view! {
                        <InlineMatchForm
                            session_id=session_id
                            characters=characters
                            selected_character_id=selected_character_id
                            on_submit=move || {
                                set_adding.set(false);
                                on_match_added();
                            }

                            on_cancel=move || set_adding.set(false)
                        />
                    }
                }
            >

                <button class="add-match-button" on:click=move |_| set_adding.set(true)>
                    "+ マッチを追加"
                </button>
            </Show>
        </div>
    }
}
