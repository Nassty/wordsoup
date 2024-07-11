use crate::result::{IError, IResult};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

use crate::{board::Board, dir::Dir};

pub struct WordPuzzle {
    nrows: isize,
    ncols: isize,
    directions: Vec<Dir>,
    words: Vec<String>,
    stack: Vec<(Board, String, Vec<usize>, Vec<Dir>)>,
    positions: Vec<usize>,
}

impl WordPuzzle {
    pub(crate) fn new(words: Vec<String>, nrows: isize, ncols: isize) -> IResult<Self> {
        let Ok(size) = usize::try_from(ncols * nrows) else {
            return Err(IError::ConversionError);
        };
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

    pub(crate) fn search(&mut self) -> IResult<Board> {
        let mut initial = Board::new(self.nrows, self.ncols)?;
        initial.fill()?;

        self.words.shuffle(&mut thread_rng());

        let Some(word) = self.words.pop() else {
            return Err(IError::BoundsError);
        };

        let mut positions = self.positions.clone();
        let mut directions = self.directions.clone();

        positions.shuffle(&mut thread_rng());
        directions.shuffle(&mut thread_rng());

        self.stack.push((initial, word, positions, directions));

        loop {
            if self.stack.is_empty() {
                return Err(IError::BoundsError);
            }

            // The top of each of stack marks the next possible set of params to search for
            let Some((current_board, word, mut positions, mut directions)) = self.stack.pop()
            else {
                return Err(IError::BoundsError);
            };

            let dir = match directions.pop() {
                None => {
                    // We have exhausted all directions for this current position
                    positions.pop();
                    // Create new directions for the next position
                    let mut directions = self.directions.clone();
                    directions.shuffle(&mut thread_rng());
                    // Return next direction
                    directions.pop().ok_or(IError::BoundsError)
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
                Some(&pos) => isize::try_from(pos).or(Err(IError::ConversionError)),
            }?;

            match try_word(&current_board, &word, pos, dir) {
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

fn try_word(board: &Board, word: &str, position: isize, direction: Dir) -> IResult<Board> {
    let mut grid = board.clone();
    let (dir_row, dir_col): (isize, isize) = direction.into();
    let (mut row, mut col) = grid.at(position);
    let mut chars: VecDeque<char> = word.chars().collect();

    while (0 <= row && row < grid.row() - 1) && (0 <= col && col < grid.cols()) {
        let Some(letter) = chars.pop_front() else {
            break;
        };

        if grid.index(row, col) == Ok('-') || grid.index(row, col) == Ok(letter) {
            grid.set(row, col, letter)?;
            row += dir_row;
            col += dir_col;
        } else {
            return Err(IError::BoundsError);
        }
    }

    if chars.is_empty() {
        return Ok(grid);
    }

    Err(IError::BoundsError)
}
