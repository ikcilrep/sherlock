use crate::board::Board;
use crate::moves::{new_move, Move};
use crate::pieces::color::get_piece_color;

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let knight_color = get_piece_color(board.pieces[from]);
    let mut to = signed_from + 17;

    if to < 64 && to & 7 > from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 15;
    if to < 64 && to & 7 < from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 17;
    if to >= 0 && to & 7 > from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 15;
    if to >= 0 && to & 7 < from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 10;
    if to < 64 && to & 7 > from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 6;
    if to < 64 && to & 7 < from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 10;
    if to >= 0 && to & 7 > from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 6;
    if to >= 0 && to & 7 < from_file && board.can_be_moved(to, knight_color) {
        result.push(new_move(from, to, board));
    }
}
