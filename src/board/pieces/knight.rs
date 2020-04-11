use crate::board::moves::Move;
use crate::board::Board;

pub fn generate_pseudo_legal_knight_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let knight_color = color!(board.pieces[from]);
    let mut to = signed_from + 17;

    if to < 64 && to & 7 > from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from + 15;
    if to < 64 && to & 7 < from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 17;
    if to >= 0 && to & 7 > from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 15;
    if to >= 0 && to & 7 < from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from + 10;
    if to < 64 && to & 7 > from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from + 6;
    if to < 64 && to & 7 < from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 10;
    if to >= 0 && to & 7 > from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }

    to = signed_from - 6;
    if to >= 0 && to & 7 < from_file && board.can_be_moved(to as usize, knight_color) {
        result.push(new_move!(from, to, board));
    }
}
