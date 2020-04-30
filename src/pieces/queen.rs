use crate::board::Board;
use crate::moves::Move;
use crate::pieces::bishop;
use crate::pieces::rook;

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    bishop::generate_pseudo_legal_moves(from, board, result);
    rook::generate_pseudo_legal_moves(from, board, result);
}
