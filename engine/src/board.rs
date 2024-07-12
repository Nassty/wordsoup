use crate::dir::Dir;
use anyhow::{bail, Context, Result};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::{fmt, iter};

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVXYZabcdefghijklmnopqrstuvxyz";

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Board {
    data: Vec<char>,
    row: isize,
    cols: isize,
    size: isize,
}
impl Board {
    pub(crate) fn new(row: isize, cols: isize) -> Result<Self> {
        let size = usize::try_from(row * cols).context("Invalid size")?;
        Ok(Self {
            data: Vec::with_capacity(size),
            row,
            cols,
            size: row * cols,
        })
    }

    #[must_use]
    pub const fn row(&self) -> isize {
        self.row
    }
    #[must_use]
    pub const fn cols(&self) -> isize {
        self.cols
    }
    #[must_use]
    pub const fn data(&self) -> &Vec<char> {
        &self.data
    }

    /// # Errors
    /// if size is invalid
    pub fn index(&self, row: isize, column: isize) -> Result<char> {
        let idx = usize::try_from(row * self.cols + column).context("Invalid index")?;
        self.data.get(idx).copied().context("Invalid index")
    }

    #[must_use]
    pub const fn at(&self, position: isize) -> (isize, isize) {
        let row = position / self.cols;
        let col = position % self.cols;
        (row, col)
    }

    pub(crate) fn set(&mut self, row: isize, column: isize, val: char) -> Result<()> {
        let idx = usize::try_from(row * self.cols + column).context("Invalid index")?;
        *self.data.get_mut(idx).context("Invalid index")? = val;
        Ok(())
    }

    pub(crate) fn fill(&mut self) -> Result<()> {
        let size = usize::try_from(self.size).context("Invalid size")?;
        self.data.extend(iter::repeat('-').take(size));
        Ok(())
    }

    /// # Errors
    ///
    /// shoudn't fail
    pub fn replace(&mut self) -> Result<()> {
        for i in 0..self.data.len() {
            if self.data[i] == '-' {
                self.data[i] = ALPHA
                    .chars()
                    .choose(&mut thread_rng())
                    .context("Invalid index")?;
            }
        }
        Ok(())
    }
    pub(crate) fn try_word(&self, word: &str, position: isize, direction: Dir) -> Result<Self> {
        let mut grid = self.clone();
        let (dir_row, dir_col): (isize, isize) = direction.into();
        let (mut row, mut col) = grid.at(position);
        let mut chars: VecDeque<char> = word.chars().collect();

        while (0 <= row && row < grid.row() - 1) && (0 <= col && col < grid.cols()) {
            let Some(letter) = chars.pop_front() else {
                break;
            };

            if grid.index(row, col)? == '-' || grid.index(row, col)? == letter {
                grid.set(row, col, letter)?;
                row += dir_row;
                col += dir_col;
            } else {
                bail!("Failed");
            }
        }

        if chars.is_empty() {
            Ok(grid)
        } else {
            bail!("Failed");
        }
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
