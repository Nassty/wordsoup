#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)] // be strict

use anyhow::{bail, Context};
mod board;
mod dir;
mod word_puzzle;
use word_puzzle::WordPuzzle;

use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        bail!("Usage: wordsoup <rows> <cols> [words..]");
    }

    let (board, words) = args.split_at(3);
    let rows = board[1].parse::<isize>().context("Invalid row size")?;
    let cols = board[2].parse::<isize>().context("Invalid column size")?;
    let mut board = WordPuzzle::new(words.to_vec(), rows, cols)?.search()?;
    board.replace()?;
    println!("{board}");
    Ok(())
}
