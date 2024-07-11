use crate::result::{IError, IResult};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::{fmt, iter};

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVXYZ";

#[derive(Clone)]
pub struct Board {
    data: Vec<char>,
    row: isize,
    cols: isize,
    size: isize,
}
impl Board {
    pub(crate) fn new(row: isize, cols: isize) -> IResult<Self> {
        let Ok(size) = usize::try_from(row * cols) else {
            return Err(IError::ConversionError);
        };
        Ok(Self {
            data: Vec::with_capacity(size),
            row,
            cols,
            size: row * cols,
        })
    }

    pub(crate) const fn row(&self) -> isize {
        self.row
    }
    pub(crate) const fn cols(&self) -> isize {
        self.cols
    }

    pub(crate) fn index(&self, row: isize, column: isize) -> IResult<char> {
        let Ok(idx) = usize::try_from(row * self.cols + column) else {
            return Err(IError::ConversionError);
        };
        Ok(self.data[idx])
    }

    #[must_use]
    pub(crate) const fn at(&self, position: isize) -> (isize, isize) {
        let row = position / self.cols;
        let col = position % self.cols;
        (row, col)
    }

    pub(crate) fn set(&mut self, row: isize, column: isize, val: char) -> IResult<()> {
        let Ok(idx) = usize::try_from(row * self.cols + column) else {
            return Err(IError::ConversionError);
        };
        self.data[idx] = val;
        Ok(())
    }

    pub(crate) fn fill(&mut self) -> IResult<()> {
        let Ok(size) = usize::try_from(self.size) else {
            return Err(IError::ConversionError);
        };
        self.data.extend(iter::repeat('-').take(size));
        Ok(())
    }

    pub(crate) fn replace(&mut self) -> IResult<()> {
        for i in 0..self.data.len() {
            if self.data[i] == '-' {
                ALPHA
                    .chars()
                    .choose(&mut thread_rng())
                    .ok_or(IError::ConversionError)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Ok(cols) = usize::try_from(self.cols()) else {
            return Err(std::fmt::Error);
        };
        writeln!(f, "-{}-", "-".repeat(cols * 3))?;
        for (index, value) in self.data.iter().enumerate() {
            if index == 0 {
                write!(f, "|")?;
            }

            if index != 0 && index % cols == 0 {
                write!(f, "|\n|")?;
            }
            write!(f, " {value} ")?;
        }
        write!(f, "|\n-{}-\n", "-".repeat(cols * 3))
    }
}
