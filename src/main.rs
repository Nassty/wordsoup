use std::env;

use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::{fmt, isize, iter};

#[derive(Clone)]
pub struct Board {
    board: Vec<char>,
    row: isize,
    cols: isize,
    size: isize,
}

const ALPHA: &str = "ABCDEFGHIJKLMNOPQRSTUVXYZ";

impl Board {
    fn new(row: isize, cols: isize) -> Self {
        Self {
            board: Vec::with_capacity((row * cols) as usize),
            row,
            cols,
            size: row * cols,
        }
    }

    pub fn index(&self, row: isize, column: isize) -> char {
        self.board[(row * self.cols + column) as usize]
    }

    pub fn at(&self, position: isize) -> (isize, isize) {
        let row = position / self.cols;
        let col = position % self.cols;
        (row, col)
    }

    pub fn set(&mut self, row: isize, column: isize, val: char) {
        self.board[(row * self.cols + column) as usize] = val;
    }

    pub fn fill(&mut self) {
        self.board
            .extend(iter::repeat('-').take(self.size as usize))
    }

    pub fn replace(&mut self) {
        for i in 0..self.board.len() {
            if self.board[i] == '-' {
                self.board[i] = ALPHA.chars().choose(&mut thread_rng()).unwrap();
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-{}-\n", "-".repeat((self.cols * 3) as usize))?;
        for (index, value) in self.board.iter().enumerate() {
            if index == 0 {
                write!(f, "|")?;
            }

            if index != 0 && index % self.cols as usize == 0 {
                write!(f, "|\n|")?;
            }
            write!(f, " {} ", value)?;
        }
        write!(f, "|\n-{}-\n", "-".repeat((self.cols * 3) as usize))
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl From<(isize, isize)> for Dir {
    fn from(position: (isize, isize)) -> Dir {
        match position {
            (0, 1) => Dir::Right,
            (0, -1) => Dir::Left,
            (-1, 0) => Dir::Up,
            (1, 0) => Dir::Down,
            (1, 1) => Dir::RightDown,
            (-1, 1) => Dir::RightUp,
            (1, -1) => Dir::LeftDown,
            (-1, -1) => Dir::LeftUp,
            _ => unreachable!(),
        }
    }
}

impl Into<(isize, isize)> for Dir {
    fn into(self) -> (isize, isize) {
        match self {
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::RightDown => (1, 1),
            Dir::RightUp => (-1, 1),
            Dir::LeftDown => (1, -1),
            Dir::LeftUp => (-1, -1),
        }
    }
}

struct WordPuzzle {
    nrows: isize,
    ncols: isize,
    directions: Vec<Dir>,
    words: Vec<String>,
    stack: Vec<(Board, String, Vec<usize>, Vec<Dir>)>,
    positions: Vec<usize>,
}

impl WordPuzzle {
    fn new(words: Vec<String>, nrows: isize, ncols: isize) -> Self {
        let positions: Vec<usize> = (0..(nrows * ncols) as usize).collect();
        Self {
            nrows,
            ncols,
            directions: vec![Dir::Right, Dir::RightDown, Dir::Down],
            words,
            stack: vec![],
            positions,
        }
    }

    pub fn search(&mut self) -> Result<Board, ()> {
        let mut initial = Board::new(self.nrows, self.ncols);
        initial.fill();

        self.words.shuffle(&mut thread_rng());

        let word = match self.words.pop() {
            Some(word) => word,
            // Empty words! WTF dude.
            None => return Err(()),
        };

        let mut positions = self.positions.clone();
        let mut directions = self.directions.clone();

        positions.shuffle(&mut thread_rng());
        directions.shuffle(&mut thread_rng());

        self.stack.push((initial, word, positions, directions));

        loop {
            if self.stack.is_empty() {
                return Err(());
            }

            // The top of each of stack marks the next possible set of params to search for
            let (current_board, word, mut positions, mut directions) = self.stack.pop().unwrap();

            let dir = match directions.pop() {
                None => {
                    // We have exhausted all directions for this current position
                    positions.pop();
                    // Create new directions for the next position
                    directions = self.directions.clone();
                    directions.shuffle(&mut thread_rng());
                    // Return next direction
                    directions.pop().unwrap()
                }
                Some(dir) => dir,
            };

            // Position we are going to search for
            let pos = match positions.last() {
                None => {
                    // We searched all positions and directions!
                    // So this didn't work.
                    self.stack.pop();
                    self.words.push(word);
                    continue;
                }
                Some(&pos) => pos as isize,
            };

            match try_word(&current_board, &word, pos as isize, dir) {
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
                Err(()) => self
                    .stack
                    .push((current_board, word, positions, directions)),
            }
        }
    }
}

fn try_word(board: &Board, word: &str, position: isize, direction: Dir) -> Result<Board, ()> {
    let mut grid = board.clone();
    let (dir_row, dir_col): (isize, isize) = direction.into();
    let (mut row, mut col) = grid.at(position);
    let mut chars: VecDeque<char> = word.chars().collect();

    while (0 <= row && row < grid.row - 1) && (0 <= col && col < grid.cols) {
        let letter = match chars.pop_front() {
            Some(c) => c,
            None => break,
        };

        if grid.index(row, col) == '-' || grid.index(row, col) == letter {
            grid.set(row, col, letter);
            row += dir_row;
            col += dir_col;
        } else {
            return Err(());
        }
    }

    if chars.is_empty() {
        return Ok(grid);
    }

    Err(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: wordsoup <rows> <cols> [words..]");
    }

    let (board, words) = args.split_at(3);
    let rows = board[1].parse::<isize>().unwrap();
    let cols = board[2].parse::<isize>().unwrap();

    let mut board = match WordPuzzle::new(words.to_vec(), rows, cols).search() {
        Ok(b) => b,
        Err(()) => {
            print!("Sorry couldn't create puzzle.");
            return;
        }
    };

    board.replace();

    println!("{}", board);
}
