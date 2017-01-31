use num::cast::NumCast;
use field::*;
use std::ops::Rem;

#[derive(Clone, Debug)]
pub struct Point<T> where T: NumCast {
    pub x: T,
    pub y: T,
}

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

impl<T: NumCast + Clone + Rem<usize, Output = U>, U: NumCast> Point<T> {
    pub fn wrap(&self, around: &Size2D<usize>) -> Point<U> {
        Point::<U> {
            x: self.x.clone() % around.width(),
            y: self.y.clone() % around.height(),
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
        assert_eq!(Direction::Right.advance(p).wrap(&rect), Point { x: 1, y: 2 });
    }
}
