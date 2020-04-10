mod board;
use crate::board::Board;

fn main() {
    let board = Board::new();
    let mut moves = Vec::new();
    let t: i8 = -1;
    println!("{}", (t as usize);
    board::pieces::rook::generate_pseudo_legal_rook_moves(0, &board, &mut moves);
    for m in moves {
        println!("Move: {}", m);
    }
}
