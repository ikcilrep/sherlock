#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod board;
mod game;
mod moves;
mod pieces;
use crate::board::Board;
use crate::game::play_random_game;

fn main() {
    let mut board = Board::new();
    let mut rng = rand::thread_rng();

    play_random_game(&mut board, &mut rng);
}
