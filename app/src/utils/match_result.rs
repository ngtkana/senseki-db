use crate::api::MatchResult;

/// 勝敗結果のボタンクラス名を生成
pub fn get_result_button_class(
    result: Option<MatchResult>,
    button_type: MatchResult,
) -> &'static str {
    match (result, button_type) {
        (Some(MatchResult::Win), MatchResult::Win) => "result-btn result-btn-win active",
        (Some(MatchResult::Loss), MatchResult::Win) => "result-btn result-btn-win inactive",
        (None, MatchResult::Win) => "result-btn result-btn-win unselected",
        (Some(MatchResult::Win), MatchResult::Loss) => "result-btn result-btn-loss inactive",
        (Some(MatchResult::Loss), MatchResult::Loss) => "result-btn result-btn-loss active",
        (None, MatchResult::Loss) => "result-btn result-btn-loss unselected",
    }
}
