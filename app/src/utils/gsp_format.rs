/// GSP値を3桁ごとにカンマ区切りでフォーマット
pub fn format_gsp(num: i32) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

/// カンマ区切りの文字列から数値のみを抽出
pub fn parse_gsp_input(input: &str) -> String {
    input.chars().filter(|c| c.is_numeric()).collect()
}

/// GSP入力値が有効かチェック
pub fn is_valid_gsp_input(input: &str) -> bool {
    if input.is_empty() {
        return true;
    }

    // 数字とカンマのみ許可
    if !input.chars().all(|c| c.is_numeric() || c == ',') {
        return false;
    }

    let numbers_only = parse_gsp_input(input);

    // パース可能かチェック
    if let Ok(num) = numbers_only.parse::<i32>() {
        // 正しいフォーマットかチェック
        let formatted = format_gsp(num);
        input == formatted
    } else {
        false
    }
}
