use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum IconSize {
    Small,  // 24px
    Medium, // 30px
    Large,  // 40px
}

impl IconSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            IconSize::Small => "character-icon-small",
            IconSize::Medium => "character-icon-medium",
            IconSize::Large => "character-icon-large",
        }
    }
}

/// 統一されたキャラクターアイコンコンポーネント
#[component]
pub fn CharacterIcon(
    fighter_key: String,
    alt: String,
    #[prop(default = IconSize::Medium)] size: IconSize,
) -> impl IntoView {
    view! {
        <img
            src=format!("/public/fighters/{}.png", fighter_key)
            class=format!("character-icon {}", size.to_class())
            alt=alt
        />
    }
}

/// 統一されたキャラクターアイコンプレースホルダー
#[component]
pub fn CharacterIconPlaceholder(
    #[prop(default = IconSize::Medium)] size: IconSize,
) -> impl IntoView {
    view! {
        <div class=format!("character-icon-placeholder {}", size.to_class())>
            "?"
        </div>
    }
}
