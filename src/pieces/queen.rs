use crate::board::Board;
use crate::moves::Move;
use crate::pieces::bishop::generate_pseudo_legal_bishop_moves;
use crate::pieces::rook::generate_pseudo_legal_rook_moves;
pub fn generate_pseudo_legal_queen_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    generate_pseudo_legal_bishop_moves(from, board, result);
    generate_pseudo_legal_rook_moves(from, board, result);
}
