use std::fmt;

use num::NumCast;

use point::*;
pub use Direction::*;
pub use Cell::*;

#[derive(Clone)]
pub enum Cell {
    Snake(Direction),
    Food,
    None
}

impl fmt::Display for Cell {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Snake(ref dir) = *self {
            dir.fmt(fmt)
        } else {
            write!(fmt, "{}", match *self {
                Food => "*",
                None => " ",
                _ => unreachable!(),
            })
        }
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
