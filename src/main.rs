#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)] // be strict

mod board;
mod dir;
mod word_puzzle;
use word_puzzle::WordPuzzle;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: wordsoup <rows> <cols> [words..]");
    }

    let (board, words) = args.split_at(3);
    let rows = board[1].parse::<isize>().unwrap();
    let cols = board[2].parse::<isize>().unwrap();

    if let Ok(mut board) = WordPuzzle::new(words.to_vec(), rows, cols)
        .unwrap()
        .search()
    {
        let _ = board.replace();
        println!("{board}");
    } else {
        print!("Sorry couldn't create puzzle.");
    }
}
