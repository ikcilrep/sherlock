use crate::board::moves::Move;
use crate::board::moves::NORMAL_MOVE;
use crate::board::Board;

pub fn generate_pseudo_legal_king_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let king_color = color!(board.pieces[from]);
    let mut to = signed_from + 7;
    if to < 64 && to & 7 < from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from + 8;
    if to < 64 && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from + 9;
    if to < 64 && to & 7 > from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from + 1;
    if to & 7 > from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 7;
    if to >= 0 && to & 7 > from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 8;
    if to >= 0 && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 9;
    if to >= 0 && to & 7 < from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 1;
    if to & 7 < from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move!(from, to, board));
    }
}
