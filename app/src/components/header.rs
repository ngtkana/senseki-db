use leptos::prelude::*;

use crate::api::Character;

#[component]
pub fn Header(
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_character_select: impl Fn(i32) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let (show_dropdown, set_show_dropdown) = signal(false);

    let selected_character = move || {
        selected_character_id
            .get()
            .and_then(|id| characters.get().iter().find(|c| c.id == id).cloned())
    };

    view! {
        <header class="header">
            <h1>"スマブラSP 戦績管理"</h1>
            <div class="header-character">
                <div
                    class="character-avatar"
                    on:click=move |_| set_show_dropdown.set(!show_dropdown.get())
                >
                    {move || {
                        if let Some(char) = selected_character() {
                            view! {
                                <img
                                    src=format!("/public/fighters/{}.png", char.fighter_key)
                                    class="avatar-icon"
                                    alt=char.name
                                />
                            }
                                .into_any()
                        } else {
                            view! { <div class="avatar-placeholder">"?"</div> }.into_any()
                        }
                    }}

                </div>

                <Show when=move || show_dropdown.get()>
                    <div class="char-dropdown-overlay" on:click=move |_| set_show_dropdown.set(false)>
                        <div
                            class="character-dropdown"
                            on:click=move |e| e.stop_propagation()
                        >
                            <div class="dropdown-header">"使用キャラを選択"</div>
                            <div class="character-grid">
                            {move || {
                                let mut chars = characters.get();
                                chars.sort_by_key(|c| c.id);
                                chars
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let is_selected = selected_character_id.get() == Some(char_id);
                                        let fighter_key = c.fighter_key.clone();
                                        let char_name = c.name.clone();
                                        let char_name_for_alt = char_name.clone();
                                        view! {
                                            <div
                                                class="character-grid-item"
                                                class:selected=move || is_selected
                                                on:click=move |_| {
                                                    on_character_select(char_id);
                                                    set_show_dropdown.set(false);
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
                            }}

                            </div>
                        </div>
                    </div>
                </Show>
            </div>
        </header>
    }
}
