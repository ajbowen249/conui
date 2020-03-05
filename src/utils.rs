/// Returns true if (`x`, `y`) is in the window at (`window_x`, `window_y`) of
/// width `width` and height `height`.
///
/// ```
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 6, 6), true);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 0, 0), false);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 12, 12), false);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 6, 12), false);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 12, 6), false);
/// ```
///
/// A coordinate is also considered "in" the window if it is anywhere on the
/// edge.
/// ```
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 11, 11), true);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 1, 1), true);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 6, 1), true);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 1, 6), true);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 11, 6), true);
/// assert_eq!(conui::is_in_window(1, 1, 10, 10, 6, 11), true);
/// ```
pub fn is_in_window(window_x: i32, window_y: i32, width: u32, height: u32, x: i32, y: i32) -> bool {
    let min_x = window_x;
    let max_x = min_x + width as i32;

    let min_y = window_y;
    let max_y = min_y + height as i32;

    !(x < min_x || x > max_x || y < min_y || y > max_y)
}
