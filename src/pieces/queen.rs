extern crate rand;

use crate::board::Board;
use crate::moves::Move;
use crate::pieces::bishop;
use crate::pieces::rook;
use rand::rngs::ThreadRng;
use rand::Rng;

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    bishop::generate_pseudo_legal_moves(from, board, result);
    rook::generate_pseudo_legal_moves(from, board, result);
}

pub fn generate_random_pseudo_legal_move(
    from: usize,
    board: &Board,
    rng: &mut ThreadRng,
) -> Option<Move> {
    if rng.gen_bool(0.5) {
        let bishop_move = bishop::generate_random_pseudo_legal_move(from, board, rng);
        if bishop_move.is_some() {
            return bishop_move;
        };
    }
    rook::generate_random_pseudo_legal_move(from, board, rng)
}

#[inline]
pub fn can_be_moved(from: usize, board: &mut Board) -> bool {
    rook::can_be_moved(from, board) || bishop::can_be_moved(from, board)
}
