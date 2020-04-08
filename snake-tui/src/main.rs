use std::{
    ops::Deref,
    sync::{Arc, Mutex, mpsc::channel},
    thread,
    time,
};

use pancurses::{initscr, endwin};
use pancurses::Window;
use pancurses::Input::*;

use snake::*;

use crate::curses_window::*;

mod curses_window;
mod snake_window;

struct MyWin(Window);

unsafe impl std::marker::Send for MyWin {}

impl Deref for MyWin {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<MyWin> for pancurses::Window {
    #[inline]
    fn into(self) -> MyWin { MyWin(self) }
}

struct Application {
    root_window: pancurses::Window,
    window: Option<Arc<Mutex<MyWin>>>,
    game: Option<Arc<Mutex<Game>>>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            root_window: initscr(),
            window: None,
            game: None,
        }
    }

    pub fn start(&mut self) {
        self.set_up();

        let (tx, rx) = channel();
        {
            let window_guard: Arc<Mutex<MyWin>> = self.window.as_ref().unwrap().clone();
            let window: &Window = &**window_guard.lock().unwrap();
            let game: Arc<Mutex<Game>> = self.game.as_ref().unwrap().clone();

            {
                let (game, tx) = (game.clone(), tx.clone());
                let g_size = game.lock().unwrap().win_size();
                let sub_win = MyWin(window.derwin(g_size.0, g_size.1, 0, 0).unwrap());

                thread::spawn(move || {
                    Application::render_loop(game, sub_win);
                    tx.send(()).unwrap();
                });
            }
            {
                let (game, tx) = (game.clone(), tx.clone());
                let window = MyWin(window.derwin(1, 1, 0, 0).unwrap());
                thread::spawn(move || {
                    Application::interaction_loop(game, window);
                    tx.send(()).unwrap();
                });
            }
        }
        // 2 threads: one for keyboard interaction, one for render.
        rx.recv().unwrap();
        rx.recv().unwrap();
    }

    fn render_loop(game: Arc<Mutex<Game>>, window: MyWin) {
        while game.lock().unwrap().state() != GameState::GameOver {
            window.clear();

            if let Ok(mut game_guard) = game.try_lock() {
                game_guard.step();
                game_guard.draw(&window);
            }

            window.refresh();
            thread::sleep(time::Duration::from_millis(500));
        }
    }

    fn interaction_loop(game: Arc<Mutex<Game>>, window: MyWin) {
        window.nodelay(false);
        window.timeout(500);
        window.keypad(true);

        while game.lock().unwrap().state() != GameState::GameOver {
            match window.getch() {
                Some(input) => match input {
                    Character('q') => {
                        game.lock().unwrap().quit();
                    },
                    key => {
                        if let Some(dir) = Direction::from_input(key) {
                            game.lock().unwrap().turn(dir).ok();
                        }
                    },
                },
                _ => (),
            }
        }
    }

    fn skip_buffered_input(&self) {
        while let Some(_input) = self.root_window.getch() {}
    }

    fn set_up(&mut self) {
        let window = &self.root_window;
        window.keypad(true);
        window.nodelay(true);
        self.skip_buffered_input();
        pancurses::noecho();
        { // colors
            use pancurses::*;

            start_color();
            init_pair(1, COLOR_RED, COLOR_BLACK);
            init_pair(2, COLOR_YELLOW, COLOR_BLACK);
            init_pair(3, COLOR_GREEN, COLOR_BLACK);
            init_pair(4, COLOR_WHITE, COLOR_BLACK);
        }
        window.erase();
        window.draw_box(0, 0);
        window.refresh();

        let width = window.get_max_x();
        let height = window.get_max_y();
        self.window =
            Some(
                Arc::new(
                    Mutex::new(
                        self.root_window
                            .derwin(height - 2, width - 2, 1, 1)
                            .unwrap()
                            .into())));

        self.game = Some(Arc::new(Mutex::new(Game::new())));
    }

    fn tear_down(&self) {
        let window = &self.root_window;
        window.keypad(false);
        window.nodelay(false);
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        self.tear_down();
        endwin();
    }
}

fn main() {
    Application::new().start();
}
