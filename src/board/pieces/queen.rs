use crate::board::moves::Move;
use crate::board::pieces::bishop::generate_pseudo_legal_bishop_moves;
use crate::board::pieces::rook::generate_pseudo_legal_rook_moves;
use crate::board::Board;
pub fn generate_pseudo_legal_queen_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    generate_pseudo_legal_bishop_moves(from, board, result);
    generate_pseudo_legal_rook_moves(from, board, result);
}
