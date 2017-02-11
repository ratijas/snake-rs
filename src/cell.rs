use std::fmt;

use num::NumCast;
#[cfg(feature = "pancurses")]
use pancurses::Input;

use point::*;
pub use Direction::*;
pub use Cell::*;

#[derive(Clone)]
pub enum Cell {
    Snake(Direction),
    Food,
    Empty
}

impl Cell {
    pub fn snake_direction(&self) -> Option<Direction> {
        match *self {
            Snake(ref dir) => Some(dir.clone()),
            _ => Option::None
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.snake_direction()
            .and_then(|dir| Some(dir.fmt(f)))
            .or_else(|| {
                Some(write!(f, "{}", match *self {
                    Food => "*",
                    Empty => " ",
                    _ => unreachable!(),
                }))
            }).unwrap()
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[cfg(feature = "pancurses")]
    pub fn from_input(key: Input) -> Option<Self> {
        Some(match key {
            Input::Character('s') | Input::KeyDown =>  Down,
            Input::Character('w') | Input::KeyUp => Up,
            Input::Character('a') | Input::KeyLeft => Left,
            Input::Character('d') | Input::KeyRight => Right,
            _ => return None,
        })
    }

    pub fn advance<T: NumCast>(&self, point: Point<T>) -> Point<T> {
        match *self {
            Up => Point { y: NumCast::from(point.y.to_isize().unwrap() - 1).unwrap(), ..point },
            Down => Point { y: NumCast::from(point.y.to_isize().unwrap() + 1).unwrap(), ..point },
            Left => Point { x: NumCast::from(point.x.to_isize().unwrap() - 1).unwrap(), ..point },
            Right => Point { x: NumCast::from(point.x.to_isize().unwrap() + 1).unwrap(), ..point },
        }
    }

    pub fn opposite(&self) -> Direction {
        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Up => "▲",
            Down => "▼",
            Left => "◀",
            Right => "▶",
        })
    }
}
