use crate::board::moves::Move;
use crate::board::pieces::sliders::add_sliding_move;
use crate::board::Board;
pub fn generate_pseudo_legal_bishop_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let bishop_color = color!(board.pieces[from]);
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let mut to = signed_from + 9;

    while to & 7 > from_file && add_sliding_move(from, to as usize, bishop_color, result, board) {
        to += 9;
    }

    to = signed_from - 9;
    while to & 7 > from_file && add_sliding_move(from, to as usize, bishop_color, result, board) {
        to -= 9;
    }

    to = signed_from + 7;
    while to & 7 < from_file && add_sliding_move(from, to as usize, bishop_color, result, board) {
        to += 7;
    }

    to = signed_from - 7;
    while to & 7 < from_file && add_sliding_move(from, to as usize, bishop_color, result, board) {
        to -= 7;
    }
}
