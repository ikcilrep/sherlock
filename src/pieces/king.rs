use crate::board::Board;
use crate::moves::constructors::{new_castling, new_move};
use crate::moves::{Move, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE};
use crate::pieces::color::get_piece_color;

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let king = board.pieces[from];
    let king_color = get_piece_color(king);
    let mut to = signed_from + 7;
    if to < 64 && to & 7 < from_file && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 8;
    if to < 64 && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 9;
    if to < 64 && to & 7 > from_file && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 1;
    if to & 7 > from_file && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 7;
    if to >= 0 && to & 7 > from_file && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 8;
    if to >= 0 && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 9;
    if to >= 0 && to & 7 < from_file && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 1;
    if to & 7 < from_file && board.can_be_moved(to, king_color) {
        result.push(new_move(from, to, board));
    }

    if board.is_castling_queens_side_pseudo_legal(king_color) {
        result.push(new_castling(CASTLING_QUEENS_SIDE, from, king, king_color))
    }

    if board.is_castling_kings_side_pseudo_legal(king_color) {
        result.push(new_castling(CASTLING_KINGS_SIDE, from, king, king_color))
    }
}
