use web_sys::KeyboardEvent;

/// キーボードナビゲーションのハンドラを生成する
pub fn handle_grid_keyboard_navigation(
    ev: &KeyboardEvent,
    current_index: i32,
    char_count: i32,
    grid_cols: i32,
    set_cursor_index: impl Fn(i32) + 'static,
    on_enter: impl Fn() + 'static,
) {
    if char_count == 0 {
        return;
    }

    match ev.key().as_str() {
        "ArrowLeft" => {
            ev.prevent_default();
            if current_index > 0 {
                set_cursor_index(current_index - 1);
            }
        }
        "ArrowRight" => {
            ev.prevent_default();
            if current_index < char_count - 1 {
                set_cursor_index(current_index + 1);
            }
        }
        "ArrowUp" => {
            ev.prevent_default();
            let new_index = current_index - grid_cols;
            if new_index >= 0 {
                set_cursor_index(new_index);
            }
        }
        "ArrowDown" => {
            ev.prevent_default();
            let new_index = current_index + grid_cols;
            if new_index < char_count {
                set_cursor_index(new_index);
            }
        }
        "Enter" => {
            ev.prevent_default();
            on_enter();
        }
        _ => {}
    }
}
