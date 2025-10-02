use leptos::prelude::*;

/// 統一されたキャラクターアイコンコンポーネント
#[component]
pub fn CharacterIcon(fighter_key: String, alt: String) -> impl IntoView {
    view! {
        <img
            src=format!("/public/fighters/{}.png", fighter_key)
            class="character-icon"
            alt=alt
        />
    }
}

/// 統一されたキャラクターアイコンプレースホルダー
#[component]
pub fn CharacterIconPlaceholder() -> impl IntoView {
    view! {
        <div class="character-icon-placeholder">
            "?"
        </div>
    }
}
