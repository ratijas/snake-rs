use std::fmt;
use std::ops::{Index, IndexMut};

use num::cast::NumCast;

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
                row.resize(width, Cell::None);
                row
            });
        }
        Field { inner: rows }
    }

    /// size is tuple `(width, height)`.
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{}", self.inner
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| format!("{}", cell))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
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
