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


pub struct Game {
    field: Field,
    tail: Point<isize>,
    head: Point<isize>,
    score: u32,
}

impl Game {
    pub fn new() -> Self {
        let (width, height) = (20, 10);
        let middle = (width / 2, height / 2);

        let mut f = Field::with_size(20, 10);
        let head = Point { x: middle.0 + 2, y: middle.1 };
        let mut p = head.clone();
        for _ in 0..4 {
            f[p.clone()] = Snake(Right);
            p.x -= 1;
        }
        let tail = Point { x: p.x + 1, ..p };

        Game {
            field: f,
            tail: tail,
            head: head,
            score: 0,
        }
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
}
