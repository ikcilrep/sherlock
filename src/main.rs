mod board;
use crate::board::Board;

fn main() {
    let board = Board::new();

    println!("Move: {}", new_move!(8, 16, &board));
}
