use std::fmt;
use std::iter;
use std::ops::{Index, IndexMut};

use num::NumCast;

use point::*;
use cell::*;


pub struct Field {
    // first y, then x
    inner: Vec<Vec<Cell>>,
}

impl Field {
    /// initialize new `Field` with given dimensions.
    pub fn with_size(width: usize, height: usize) -> Self {
        let mut rows = Vec::with_capacity(height);
        for _ in 0..height {
            rows.push({
                let mut row = Vec::with_capacity(width);
                row.resize(width, Cell::Empty);
                row
            });
        }
        Field { inner: rows }
    }

    /// put a snake on the game field.
    /// returns its head and tail positions.
    pub fn init_snake<T>(&mut self, len: usize) -> (Point<T>, Point<T>)
    where T: NumCast + Clone {
        assert!(len <= self.width());

        let head = Point::<T> {
            x: NumCast::from((self.width() + len) / 2).unwrap(),
            y: NumCast::from(self.height() / 2).unwrap(),
        };
        let mut tail = head.clone();
        for _ in 0..len {
            self[tail.clone()] = Snake(Right);
            tail.x = NumCast::from(tail.x.to_isize().unwrap() - 1).unwrap();
        }
        tail.x = NumCast::from(tail.x.to_isize().unwrap() + 1).unwrap();
        (head, tail)
    }

    /// size is tuple `(width, height)`.
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn rows(&self) -> &Vec<Vec<Cell>> { &self.inner }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let h_border = format!("+{}+",
                               iter::repeat("-")
                                   .take(self.width())
                                   .collect::<String>());
        writeln!(
            f, "{}\n{}\n{}",
            h_border,
            self.inner
                .iter()
                .map(|row| {
                    format!("|{}|",
                            row.iter()
                                .map(|cell| format!("{}", cell))
                                .collect::<Vec<_>>()
                                .join(""))
                })
                .collect::<Vec<_>>()
                .join("\n"),
            h_border
        )
    }
}

impl Size2D<usize> for Field {
    #[inline]
    fn width(&self) -> usize { self.inner[0].len() }

    #[inline]
    fn height(&self) -> usize { self.inner.len() }
}

impl Index<(usize, usize)> for Field {
    type Output = Cell;

    /// `index` is tuple of (x, y).  coordinates are zero-based.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.inner[index.1][index.0]
    }
}

impl IndexMut<(usize, usize)> for Field {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.inner[index.1][index.0]
    }
}

impl<T: NumCast> Index<Point<T>> for Field {
    type Output = Cell;

    fn index(&self, index: Point<T>) -> &Self::Output {
        &self.inner[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}

impl<T: NumCast> IndexMut<Point<T>> for Field {
    fn index_mut(&mut self, index: Point<T>) -> &mut Self::Output {
        &mut self.inner[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}

impl<'a, T: NumCast> Index<&'a Point<T>> for Field {
    type Output = Cell;

    fn index(&self, index: &'a Point<T>) -> &Self::Output {
        &self.inner[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}

impl<'a, T: NumCast> IndexMut<&'a Point<T>> for Field {
    fn index_mut(&mut self, index: &Point<T>) -> &mut Self::Output {
        &mut self.inner[index.y.to_usize().unwrap()][index.x.to_usize().unwrap()]
    }
}
