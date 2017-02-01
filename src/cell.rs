use std::fmt;

use num::NumCast;

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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.snake_direction()
            .and_then(|dir| Some(dir.fmt(fmt)))
            .or_else(|| {
                Some(write!(fmt, "{}", match *self {
                    Food => "*",
                    Empty => " ",
                    _ => unreachable!(),
                }))
            }).unwrap()
    }
}


#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn advance<T: NumCast>(&self, point: Point<T>) -> Point<T> {
        match *self {
            Up => Point { y: NumCast::from(point.y.to_isize().unwrap() - 1).unwrap(), ..point },
            Down => Point { y: NumCast::from(point.y.to_isize().unwrap() + 1).unwrap(), ..point },
            Left => Point { x: NumCast::from(point.x.to_isize().unwrap() - 1).unwrap(), ..point },
            Right => Point { x: NumCast::from(point.x.to_isize().unwrap() + 1).unwrap(), ..point },
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", match *self {
            Up => "^",
            Down => "v",
            Left => "<",
            Right => ">",
        })
    }
}
