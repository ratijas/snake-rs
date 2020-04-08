use pancurses::{self, Window, COLOR_PAIR};

use snake::*;

use crate::curses_window::CursesWindow;

impl CursesWindow for Game {
    fn win_size(&self) -> (i32, i32) {
        let view: FieldView = self.into();
        let (height, width) = view.win_size();
        (height + 2 + 1, width + 2) // 2 for border and 1 for header
    }

    fn draw(&self, window: &Window) {
        window.clear();
        window.draw_box(0, 0);
        window.mvprintw(1, 2, &format!("Score: {}", self.score()));
        // field's sub-window
        {
            let view: FieldView = self.into();
            let (height, width) = view.win_size();
            let sub_window = window.derwin(height, width, 2, 1).unwrap();
            view.draw(&sub_window);
            sub_window.delwin();
        }
    }
}

struct FieldView<'a> {
    field: &'a Field,
    head: Point<isize>,
    tail: Point<isize>,
}

impl<'a> FieldView<'a> {
    pub fn new(field: &'a Field, head: Point<isize>, tail: Point<isize>) -> Self {
        Self {
            field,
            head,
            tail
        }
    }
}

impl<'a> From<&'a Game> for FieldView<'a> {
    fn from(game: &'a Game) -> Self {
        Self::new(game.field(), game.head(), game.tail())
    }
}

impl<'a> CursesWindow for FieldView<'a> {
    fn win_size(&self) -> (i32, i32) {
        let (width, height) = self.field.size();
        (height as i32 + 2, width as i32 + 2)
    }

    fn draw(&self, window: &Window) {
        window.clear();
        window.draw_box(0, 0);

        for (y, row) in self.field.rows().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // snake parts are drawn separately
                if !matches!(*cell, Cell::Snake(_)) {
                    window.mvprintw(1+y as i32, 1+x as i32, &format!("{}", cell));
                }
            }
        }
        // now it's snake time, from tail to head.
        let snake = {
            let mut it = Vec::new();
            let mut p = self.tail.clone();
            it.push(p);
            loop {
                // prevent infinite loop
                if p == self.head { break; }
                // safely unwrap because we are not yet at the head
                p = self.field.next_point(p).expect("no snake part here");
                it.push(p);
            }
            it
        };
        let colors = [
            COLOR_PAIR(4),
            COLOR_PAIR(3),
            COLOR_PAIR(2) | pancurses::A_BOLD,
            COLOR_PAIR(1),
        ];
        for (p, color) in snake.iter().rev().zip(colors.iter().cycle()) {
            window.attron(*color);
            window.mvprintw(1 + p.y as i32,
                            1 + p.x as i32,
                            &format!("{}", self.field[p]));
            window.attroff(*color);
        }
        window.attroff(colors[0]);
    }
}