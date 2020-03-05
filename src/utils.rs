use std::rc::Rc;
use std::cell::RefCell;

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

/// Wrapper around Rc<RefCell<String>>
/// Allows for logic to operate on text data without needing to keep tight
/// connections between componenets.
///
/// ```
/// let mut binder1 = conui::TextBinder::new("test123");
/// assert_eq!(binder1.get(), "test123");
///
/// let binder2 = binder1.clone();
/// binder1.set("something else");
/// assert_eq!(binder2.get(), "something else");
/// ```
#[derive(Clone)]
pub struct TextBinder {
    text: Rc<RefCell<String>>,
}

impl TextBinder {
    pub fn new(initial_text: &str) -> TextBinder {
        TextBinder {
            text: Rc::new(RefCell::new(String::from(initial_text))),
        }
    }

    pub fn set(&mut self, text: &str) {
        *self.text.borrow_mut() = String::from(text);
    }

    pub fn get(&self) -> String {
        (*self.text.borrow()).clone()
    }
}
