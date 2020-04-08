extern crate pancurses;

use pancurses::Window;

/// A trait for objects that can render themselves on an ncurses `Window`.
pub trait CursesWindow {
    /// suggested size to create ncurses `Window` for this widget.
    /// returned value interpreted as `(height, width)` pair.
    fn win_size(&self) -> (i32, i32);

    /// render self on given window.
    fn draw(&self, window: &Window);
}
