use crate::api::Character;

/// 検索用に文字列を正規化する（小文字化、空白除去）
pub fn normalize_for_search(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// キャラクターが検索クエリにマッチするかチェック
pub fn matches_search(character: &Character, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let normalized_query = normalize_for_search(query);

    // 日本語名で検索
    if normalize_for_search(&character.name).contains(&normalized_query) {
        return true;
    }

    // 英語名で検索
    if normalize_for_search(&character.name_en).contains(&normalized_query) {
        return true;
    }

    // fighter_keyでも検索（内部キー）
    if normalize_for_search(&character.fighter_key).contains(&normalized_query) {
        return true;
    }

    false
}
