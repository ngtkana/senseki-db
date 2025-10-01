use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{self, Character, CreateMatchRequest};

#[component]
pub fn InlineMatchForm(
    session_id: i32,
    characters: ReadSignal<Vec<Character>>,
    selected_character_id: ReadSignal<Option<i32>>,
    on_submit: impl Fn() + 'static + Copy,
    on_cancel: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let (character_id, set_character_id) = signal(selected_character_id.get().unwrap_or(0));
    let (opponent_id, set_opponent_id) = signal(0);
    let (result, set_result) = signal(String::from("win"));
    let (comment, set_comment) = signal(String::new());
    let (loading, set_loading) = signal(false);

    let handle_submit = move |ev: leptos::web_sys::SubmitEvent| {
        ev.prevent_default();

        if character_id.get() == 0 || opponent_id.get() == 0 {
            logging::error!("使用キャラ、相手キャラを選択してください");
            return;
        }

        set_loading.set(true);

        let req = CreateMatchRequest {
            session_id,
            character_id: character_id.get(),
            opponent_character_id: opponent_id.get(),
            result: result.get(),
            comment: if comment.get().is_empty() {
                None
            } else {
                Some(comment.get())
            },
        };

        spawn_local(async move {
            match api::create_match(req).await {
                Ok(_) => {
                    logging::log!("マッチ記録成功");
                    on_submit();
                }
                Err(e) => {
                    logging::error!("マッチ記録失敗: {}", e);
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="add-match-form">
            <form on:submit=handle_submit>
                <div class="form-row">
                    <div class="form-group">
                        <label>"使用キャラ"</label>
                        <select
                            class="form-input"
                            on:change=move |ev| {
                                set_character_id.set(event_target_value(&ev).parse().unwrap_or(0))
                            }
                        >

                            <option value="0" selected=move || character_id.get() == 0>
                                "選択"
                            </option>
                            {move || {
                                characters
                                    .get()
                                    .iter()
                                    .map(|c| {
                                        let char_id = c.id;
                                        let is_selected = move || character_id.get() == char_id;
                                        view! {
                                            <option value=char_id selected=is_selected>
                                                {c.name.clone()}
                                            </option>
                                        }
                                    })
                                    .collect_view()
                            }}

                        </select>
                    </div>
                    <div class="form-group">
                        <label>"相手キャラ"</label>
                        <select
                            class="form-input"
                            on:change=move |ev| {
                                set_opponent_id.set(event_target_value(&ev).parse().unwrap_or(0))
                            }
                        >

                            <option value="0">"選択"</option>
                            {move || {
                                characters
                                    .get()
                                    .iter()
                                    .map(|c| view! { <option value=c.id>{c.name.clone()}</option> })
                                    .collect_view()
                            }}

                        </select>
                    </div>
                </div>

                <div class="form-group">
                    <label>"結果"</label>
                    <div class="radio-group">
                        <label class="radio-label">
                            <input
                                type="radio"
                                name="result"
                                value="win"
                                checked=move || result.get() == "win"
                                on:change=move |_| set_result.set("win".to_string())
                            />
                            " 勝ち"
                        </label>
                        <label class="radio-label">
                            <input
                                type="radio"
                                name="result"
                                value="loss"
                                checked=move || result.get() == "loss"
                                on:change=move |_| set_result.set("loss".to_string())
                            />
                            " 負け"
                        </label>
                    </div>
                </div>

                <div class="form-group">
                    <label>"コメント"</label>
                    <textarea
                        class="form-input"
                        placeholder="良い試合だった、など..."
                        on:input=move |ev| set_comment.set(event_target_value(&ev))
                        prop:value=comment
                    />
                </div>

                <div class="form-actions">
                    <button type="button" class="btn" on:click=move |_| on_cancel()>
                        "キャンセル"
                    </button>
                    <button type="submit" class="btn btn-primary" disabled=loading>
                        {move || if loading.get() { "記録中..." } else { "追加" }}
                    </button>
                </div>
            </form>
        </div>
    }
}
