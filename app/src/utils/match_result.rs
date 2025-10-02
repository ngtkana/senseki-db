/// 勝敗結果のボタンクラス名を生成
pub fn get_result_button_class(result: &str, button_type: &str) -> &'static str {
    match (result, button_type) {
        ("win", "win") => "result-btn result-btn-win active",
        ("loss", "win") => "result-btn result-btn-win inactive",
        ("", "win") => "result-btn result-btn-win unselected",
        ("win", "loss") => "result-btn result-btn-loss inactive",
        ("loss", "loss") => "result-btn result-btn-loss active",
        ("", "loss") => "result-btn result-btn-loss unselected",
        _ => "result-btn",
    }
}
