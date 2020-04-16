use crate::board::moves::Move;
use crate::board::pieces::color::get_piece_color;
use crate::board::pieces::sliders::add_sliding_move;
use crate::board::Board;

pub fn generate_pseudo_legal_rook_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let rook_color = get_piece_color(board.pieces[from]);
    let signed_from = from as i8;
    let mut to = signed_from + 1;
    while to & 7 != 0 && add_sliding_move(from, to as usize, rook_color, result, board) {
        to += 1;
    }

    to = signed_from - 1;
    while to & 7 != 7 && add_sliding_move(from, to as usize, rook_color, result, board) {
        to -= 1;
    }

    to = signed_from + 8;
    while to < 64 && add_sliding_move(from, to as usize, rook_color, result, board) {
        to += 8;
    }

    to = signed_from - 8;
    while to >= 0 && add_sliding_move(from, to as usize, rook_color, result, board) {
        to -= 8;
    }
}
