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
