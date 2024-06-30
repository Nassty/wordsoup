use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use std::fmt;

const ALPHA: &[u8] = ".".as_bytes(); // "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

pub struct Board {
    board: Vec<Vec<char>>,
    size: usize,
}

impl Board {
    fn new(size: usize) -> Self {
        let mut board = Vec::with_capacity(size);
        for _ in 0..size {
            board.push(Vec::with_capacity(size));
        }
        Self { board, size }
    }

    pub fn fill(&mut self) {
        let mut rng = rand::thread_rng();
        for row in self.board.iter_mut() {
            for _index in 0..self.size {
                row.push(*ALPHA.choose(&mut rng).unwrap() as char);
            }
        }
    }

    pub fn place_horizontal_word(&mut self, row: usize, col: usize, word: &str) {
        self.board[row].splice(col..col + word.len(), word.chars());
    }

    pub fn place_vertical_word(&mut self, row: usize, col: usize, word: &str) {
        let mut current_row = row;
        for c in word.chars() {
            self.board[current_row][col] = c;
            current_row += 1;
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-{}-\n", "-".repeat(self.size * 3))?;
        for row in self.board.iter() {
            write!(f, "|")?;
            for v in row.iter() {
                write!(f, " {} ", v)?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "-{}-\n", "-".repeat(self.size * 3))
    }
}

fn main() {
    //
    // let end = self.size - word.len();
    // let between = Uniform::new(0, end);
    // let mut rng = rand::thread_rng();
    // let start = between.sample(&mut rng);
    let mut board = Board::new(13);
    board.fill();
    board.place_horizontal_word(5, 3, "GATO");
    board.place_vertical_word(4, 4, "PATO");
    println!("{}", board);
}
