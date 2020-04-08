use pancurses::{self, Window, COLOR_PAIR};

use snake::*;

use crate::curses_window::CursesWindow;

impl CursesWindow for Game {
    fn win_size(&self) -> (i32, i32) {
        let (height, width) = self.field().win_size();
        (height + 2 + 1, width + 2) // 2 for border and 1 for header
    }

    fn draw(&self, window: &Window) {
        window.clear();
        window.draw_box(0, 0);
        window.mvprintw(1, 2, &format!("your score: {}", self.score()));
        {
            let (height, width) = self.field().win_size();
            let window = window.derwin(height, width, 2, 1).unwrap();
            window.clear();
            window.draw_box(0, 0);
            let snake = {
                let mut snake = Vec::new();
                let mut p = self.tail();
                snake.push(p);
                loop {
                    p = self.field().next_point(p).unwrap();
                    snake.push(p);
                    if p == self.head() { break; }
                }
                snake
            };

            self.field()
                .rows()
                .iter()
                .enumerate()
                .fold((), |_, (y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|&(_, cell)| match *cell { Cell::Snake(_) => false, _ => true})
                        .fold((), |_, (x, cell)| {
                            window.mvprintw(1+y as i32, 1+x as i32, &format!("{}", cell));
                        });
                });

            let colors = vec![
                COLOR_PAIR(4),
                COLOR_PAIR(3),
                COLOR_PAIR(2) | pancurses::A_BOLD,
                COLOR_PAIR(1),
            ];

            for (p, &color) in snake.iter().zip(
                colors
                    .iter()
                    .cycle()
                    .skip(colors.len() - self.snake_len() % colors.len())) {
                window.attron(color);
                window.mvprintw(1 + p.y as i32,
                                1 + p.x as i32,
                                &format!("{}", self.field()[p]));
                window.attroff(color);
            }
            window.attroff(colors[0]);

            window.delwin();
        }
    }
}

impl CursesWindow for Field {
    fn win_size(&self) -> (i32, i32) {
        let (width, height) = self.size();
        (height as i32 + 2, width as i32 + 2)
    }

    fn draw(&self, window: &Window) {
        window.clear();
        window.draw_box(0, 0);
/*
        self.rows()
            .iter()
            .enumerate()
            .fold((), |_, (y, row)| {
                row.iter()
                    .enumerate()
                    .fold((), |_, (x, cell)| {
                        window.mvprintw(1+y as i32, 1+x as i32, &format!("{}", cell));
                    });
            });
*/
    }
}