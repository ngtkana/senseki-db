use leptos::logging;
use leptos::prelude::*;
use leptos::web_sys;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (show_session_modal, set_show_session_modal) = signal(false);
    let (show_match_modal, set_show_match_modal) = signal(false);

    view! {
        <div class="app">
            <Header/>
            <main class="container">
                <HomePage
                    on_new_session=move || set_show_session_modal.set(true)
                    on_new_match=move || set_show_match_modal.set(true)
                />
            </main>

            <Show when=move || show_session_modal.get()>
                <Modal on_close=move || set_show_session_modal.set(false)>
                    <SessionForm on_submit=move || set_show_session_modal.set(false)/>
                </Modal>
            </Show>

            <Show when=move || show_match_modal.get()>
                <Modal on_close=move || set_show_match_modal.set(false)>
                    <MatchForm on_submit=move || set_show_match_modal.set(false)/>
                </Modal>
            </Show>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <h1>"🎮 スマブラSP 戦績管理"</h1>
            <nav class="nav">
                <a href="#" class="nav-link active">"ホーム"</a>
                <a href="#" class="nav-link">"セッション"</a>
                <a href="#" class="nav-link">"統計"</a>
            </nav>
        </header>
    }
}

#[component]
fn HomePage(
    on_new_session: impl Fn() + 'static + Copy,
    on_new_match: impl Fn() + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="home">
            <section class="stats-card">
                <h2>"📊 今日の戦績"</h2>
                <div class="stats-grid">
                    <div class="stat-item">
                        <div class="stat-label">"使用キャラ"</div>
                        <div class="stat-value">"カービィ"</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-label">"戦績"</div>
                        <div class="stat-value">"5勝3敗"</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-label">"勝率"</div>
                        <div class="stat-value">"62.5%"</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-label">"GSP"</div>
                        <div class="stat-value">"10,500,000"</div>
                    </div>
                </div>
            </section>

            <section class="actions">
                <button
                    class="btn btn-primary"
                    on:click=move |_| on_new_session()
                >
                    "+ 新しいセッション"
                </button>
                <button
                    class="btn btn-secondary"
                    on:click=move |_| on_new_match()
                >
                    "+ マッチを記録"
                </button>
            </section>

            <section class="recent-sessions">
                <h2>"📅 最近のセッション"</h2>
                <div class="session-list">
                    <SessionCard
                        date="2025-10-02"
                        note="カービィで練習"
                        matches=8
                    />
                    <SessionCard
                        date="2025-10-01"
                        note="ランク上げ"
                        matches=12
                    />
                </div>
            </section>
        </div>
    }
}

#[component]
fn SessionCard(date: &'static str, note: &'static str, matches: i32) -> impl IntoView {
    view! {
        <div class="session-card">
            <div class="session-date">{date}</div>
            <div class="session-note">{note}</div>
            <div class="session-matches">{matches}" 戦"</div>
        </div>
    }
}

#[component]
fn Modal(on_close: impl Fn() + 'static + Copy, children: Children) -> impl IntoView {
    view! {
        <div class="modal-overlay" on:click=move |_| on_close()>
            <div class="modal-content" on:click=|e| e.stop_propagation()>
                <button class="modal-close" on:click=move |_| on_close()>
                    "×"
                </button>
                {children()}
            </div>
        </div>
    }
}

#[component]
fn SessionForm(on_submit: impl Fn() + 'static + Copy) -> impl IntoView {
    let (notes, set_notes) = signal(String::new());

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        logging::log!("セッション作成: {}", notes.get());
        on_submit();
    };

    view! {
        <div class="form-container">
            <h2>"新しいセッション"</h2>
            <form on:submit=handle_submit>
                <div class="form-group">
                    <label>"日付"</label>
                    <input
                        type="date"
                        class="form-input"
                        value=chrono::Local::now().format("%Y-%m-%d").to_string()
                    />
                </div>
                <div class="form-group">
                    <label>"メモ"</label>
                    <textarea
                        class="form-input"
                        placeholder="今日の目標や気をつけること..."
                        on:input=move |ev| set_notes.set(event_target_value(&ev))
                        prop:value=notes
                    />
                </div>
                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">
                        "作成"
                    </button>
                </div>
            </form>
        </div>
    }
}

#[component]
fn MatchForm(on_submit: impl Fn() + 'static + Copy) -> impl IntoView {
    let (character, set_character) = signal(String::new());
    let (opponent, set_opponent) = signal(String::new());
    let (result, set_result) = signal(String::from("win"));
    let (gsp_before, set_gsp_before) = signal(String::new());
    let (gsp_after, set_gsp_after) = signal(String::new());
    let (comment, set_comment) = signal(String::new());

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        logging::log!(
            "マッチ記録: {} vs {} = {}",
            character.get(),
            opponent.get(),
            result.get()
        );
        on_submit();
    };

    view! {
        <div class="form-container">
            <h2>"マッチを記録"</h2>
            <form on:submit=handle_submit>
                <div class="form-row">
                    <div class="form-group">
                        <label>"使用キャラ"</label>
                        <input
                            type="text"
                            class="form-input"
                            placeholder="カービィ"
                            on:input=move |ev| set_character.set(event_target_value(&ev))
                            prop:value=character
                        />
                    </div>
                    <div class="form-group">
                        <label>"相手キャラ"</label>
                        <input
                            type="text"
                            class="form-input"
                            placeholder="マリオ"
                            on:input=move |ev| set_opponent.set(event_target_value(&ev))
                            prop:value=opponent
                        />
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

                <div class="form-row">
                    <div class="form-group">
                        <label>"GSP（開始）"</label>
                        <input
                            type="number"
                            class="form-input"
                            placeholder="10000000"
                            on:input=move |ev| set_gsp_before.set(event_target_value(&ev))
                            prop:value=gsp_before
                        />
                    </div>
                    <div class="form-group">
                        <label>"GSP（終了）"</label>
                        <input
                            type="number"
                            class="form-input"
                            placeholder="10050000"
                            on:input=move |ev| set_gsp_after.set(event_target_value(&ev))
                            prop:value=gsp_after
                        />
                    </div>
                </div>

                <div class="form-group">
                    <label>"コメント"</label>
                    <textarea
                        class="form-input"
                        placeholder="良い試合だった、ラグかった、など..."
                        on:input=move |ev| set_comment.set(event_target_value(&ev))
                        prop:value=comment
                    />
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">
                        "記録"
                    </button>
                </div>
            </form>
        </div>
    }
}
