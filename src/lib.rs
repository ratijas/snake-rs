#![allow(dead_code)]
#![allow(unused_imports)]

extern crate num;

mod field;
mod point;
mod cell;

use std::fmt;

pub use point::*;
pub use field::*;
pub use cell::*;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    GameOn,
    GameOver,
}

pub struct Game {
    field: Field,
    tail: Point<isize>,
    head: Point<isize>,
    score: u32,
    snake_len: u32,
    state: GameState,
    no_turn_back: Direction,
}

impl Game {
    pub fn new() -> Self {
        let (width, height) = (20, 10);
        let mut f = Field::with_size(width, height);
        let (head, tail) = f.init_snake::<isize>(5);
        Game {
            field: f,
            tail: tail,
            head: head,
            score: 0,
            snake_len: 5,
            state: GameState::GameOn,
            no_turn_back: Direction::Left,
        }
    }

    pub fn step(&mut self) -> GameState {
        if self.state == GameState::GameOver { return self.state }

        let next_point = self.field[&self.head]
            .clone()
            .snake_direction()
            .unwrap()
            .advance(self.head.clone())
            .wrap(&self.field.size());
        let next_cell = self.field[&next_point].clone();

        match next_cell {
            Empty => {
                self.move_tail();
                self.move_head()
            },
            Snake(_) if next_point == self.tail => {
                self.move_tail();
                self.move_head()
            },
            Snake(_) => {
                self.state = GameState::GameOver
            },
            Food => self.move_head(),
        }
        self.state
    }

    fn move_head(&mut self) {
        let head: Cell = self.field[&self.head].clone();
        let direction = head.snake_direction().unwrap();
        let head_next = direction.advance(self.head.clone()).wrap(&self.field.size());
        self.field[&head_next] = Cell::Snake(direction);
        self.head = head_next;
        self.no_turn_back = direction.opposite();
    }

    fn move_tail(&mut self) {
        let tail = self.field[&self.tail].clone();
        self.field[&self.tail] = Cell::Empty;
        self.tail = tail.snake_direction().unwrap().advance(self.tail.clone()).wrap(&self.field.size());
    }

    pub fn state(&self) -> GameState { self.state }

    pub fn quit(&mut self) { self.state = GameState::GameOver; }

    pub fn field(&self) -> &Field { &self.field }

    pub fn score(&self) -> u32 { self.score }

    pub fn turn(&mut self, dir: Direction) -> Result<(), ()> {
        if self.state == GameState::GameOver { return Err(()) }
        if self.no_turn_back == dir { return Err(()) }
        match self.field[&self.head] {
            Cell::Snake(ref mut d) => *d = dir,
            _ => unreachable!()
        }
        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, " score: {}", self.score)?;
        writeln!(fmt, "{}", self.field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let f = Field::with_size(3, 5);
        assert_eq!(f.size(), (3, 5));
    }
    #[test]
    fn print_game() {
        let game = Game::new();
        println!("{}", game);
    }

    #[test]
    fn change_direction() {
        let mut game = Game::new();
        game.turn(Direction::Down).unwrap();
        game.step();
        assert_eq!(game.field[&game.head].snake_direction().unwrap(), Direction::Down);
        assert_eq!(game.no_turn_back, Direction::Up);
        assert_eq!(game.turn(Direction::Up), Err(()));
    }
}
