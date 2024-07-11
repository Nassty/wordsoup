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
    pub(crate) fn new(row: isize, cols: isize) -> Self {
        Self {
            data: Vec::with_capacity((row * cols) as usize),
            row,
            cols,
            size: row * cols,
        }
    }

    pub(crate) const fn row(&self) -> isize {
        self.row
    }
    pub(crate) const fn cols(&self) -> isize {
        self.cols
    }

    #[must_use]
    pub(crate) fn index(&self, row: isize, column: isize) -> char {
        self.data[(row * self.cols + column) as usize]
    }

    #[must_use]
    pub(crate) const fn at(&self, position: isize) -> (isize, isize) {
        let row = position / self.cols;
        let col = position % self.cols;
        (row, col)
    }

    pub(crate) fn set(&mut self, row: isize, column: isize, val: char) {
        self.data[(row * self.cols + column) as usize] = val;
    }

    pub(crate) fn fill(&mut self) {
        self.data.extend(iter::repeat('-').take(self.size as usize));
    }

    pub(crate) fn replace(&mut self) -> Result<(), ()> {
        for i in 0..self.data.len() {
            if self.data[i] == '-' {
                ALPHA.chars().choose(&mut thread_rng()).ok_or(())?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "-{}-", "-".repeat((self.cols * 3) as usize))?;
        for (index, value) in self.data.iter().enumerate() {
            if index == 0 {
                write!(f, "|")?;
            }

            if index != 0 && index % self.cols as usize == 0 {
                write!(f, "|\n|")?;
            }
            write!(f, " {value} ")?;
        }
        write!(f, "|\n-{}-\n", "-".repeat((self.cols * 3) as usize))
    }
}
