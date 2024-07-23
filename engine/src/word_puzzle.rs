use anyhow::{bail, Context, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{board::Board, dir::Dir};

#[derive(PartialEq)]
pub struct WordPuzzle {
    nrows: isize,
    ncols: isize,
    directions: Vec<Dir>,
    words: Vec<String>,
    stack: Vec<(Board, String, Vec<usize>, Vec<Dir>)>,
    positions: Vec<usize>,
}

impl WordPuzzle {
    /// # Errors
    /// fails if ncols*nrows is not a valid usize
    pub fn new(words: Vec<String>, nrows: isize, ncols: isize) -> Result<Self> {
        let size = usize::try_from(ncols * nrows).context("Invalid size")?;
        let positions: Vec<usize> = (0..size).collect();
        Ok(Self {
            nrows,
            ncols,
            directions: vec![Dir::Right, Dir::RightDown, Dir::Down],
            words,
            stack: vec![],
            positions,
        })
    }
    /// # Errors
    /// fails if no words are left to search while generating
    pub fn search(&mut self) -> Result<Board> {
        let mut initial = Board::new(self.nrows, self.ncols)?;
        initial.fill()?;

        self.words.shuffle(&mut thread_rng());

        let word = self.words.pop().context("No words to search")?;

        let mut positions = self.positions.clone();
        let mut directions = self.directions.clone();

        positions.shuffle(&mut thread_rng());
        directions.shuffle(&mut thread_rng());

        self.stack.push((initial, word, positions, directions));

        loop {
            if self.stack.is_empty() {
                bail!("No solution found");
            }

            // The top of each of stack marks the next possible set of params to search for
            let Some((current_board, word, mut positions, mut directions)) = self.stack.pop()
            else {
                bail!("No solution found");
            };

            let dir = match directions.pop() {
                None => {
                    // We have exhausted all directions for this current position
                    positions.pop();
                    // Create new directions for the next position
                    let mut directions = self.directions.clone();
                    directions.shuffle(&mut thread_rng());
                    // Return next direction
                    directions.pop().context("No directions left")
                }
                Some(dir) => Ok(dir),
            }?;

            // Position we are going to search for
            let pos = match positions.last() {
                None => {
                    // We searched all positions and directions!
                    // So this didn't work.
                    self.stack.pop();
                    self.words.push(word);
                    continue;
                }
                Some(&pos) => isize::try_from(pos).context("Invalid position"),
            }?;

            match current_board.try_word(&word, pos, dir) {
                Ok(board) => {
                    let word = match self.words.pop() {
                        None => return Ok(board),
                        Some(word) => word,
                    };

                    let mut positions = self.positions.clone();
                    positions.shuffle(&mut thread_rng());

                    let mut directions = self.directions.clone();
                    directions.shuffle(&mut thread_rng());

                    self.stack.push((board, word, positions, directions));
                }
                // If a I do a last() to check the top of the stack
                // I have to use references
                Err(_) => self
                    .stack
                    .push((current_board, word, positions, directions)),
            }
        }
    }
}
