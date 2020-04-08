use num::{cast, NumCast};

#[derive(Clone, Debug)]
pub struct Point<T> where T: NumCast {
    pub x: T,
    pub y: T,
}

impl<T> Copy for Point<T> where T: NumCast + Copy {}

impl<T> PartialEq for Point<T> where T: NumCast + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: NumCast> From<(T, T)> for Point<T> {
    fn from(tuple: (T, T)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

pub trait Size2D<T> {
    fn width(&self) -> T;
    fn height(&self) -> T;
}

impl<T> Size2D<T> for (T, T) where T: Clone {
    fn width(&self) -> T { self.0.clone() }
    fn height(&self) -> T { self.1.clone() }
}

macro_rules! modulus (
    ($a:expr, $b:expr) => {{
        let a = $a as isize;
        let b = $b as isize;
        ((a % b) + b) % b
    }}
);

impl<T: NumCast> Point<T> {
    pub fn wrap(&self, around: &dyn Size2D<usize>) -> Self {
        Point {
            x: cast(modulus!(self.x.to_isize().unwrap(), around.width())).unwrap(),
            y: cast(modulus!(self.y.to_isize().unwrap(), around.height())).unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    #[test]
    fn test_wrap() {
        let p = Point { x: 4, y: 6 };
        let rect = (2usize, 4usize);
        assert_eq!(p.wrap(&rect), Point { x: 0, y: 2 });
        assert_eq!(Direction::Right.advance(p.clone()).wrap(&rect), Point { x: 1, y: 2 });
    }

    #[test]
    fn test_wrap_negative() {
        assert_eq!(
            Point::from((-1, -2)).wrap(&(4, 3)),
            Point::from((3, 1)));
    }
}
