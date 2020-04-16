use crate::board::Board;
use crate::moves::{new_castling, new_move, Move, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE};
use crate::pieces::color::get_piece_color;
use crate::pieces::EMPTY_SQUARE;

const KNIGHTS_KINGS_SIDE_POSITIONS: [usize; 2] = [6, 62];
const KNIGHTS_QUEENS_SIDE_POSITIONS: [usize; 2] = [1, 57];
const BISHOP_KINGS_SIDE_POSITIONS: [usize; 2] = [5, 61];
const BISHOP_QUEENS_SIDE_POSITIONS: [usize; 2] = [2, 58];
const QUEENS_POSITIONS: [usize; 2] = [3, 59];

macro_rules! squares_between_king_and_queens_rook_are_not_occupied {
    ($color: expr, $board: expr) => {
        ($board).pieces[BISHOP_QUEENS_SIDE_POSITIONS[($color) as usize]] == EMPTY_SQUARE
            && ($board).pieces[KNIGHTS_QUEENS_SIDE_POSITIONS[($color) as usize]] == EMPTY_SQUARE
            && ($board).pieces[QUEENS_POSITIONS[($color) as usize]] == EMPTY_SQUARE
    };
}

macro_rules! squares_between_king_and_kings_rook_are_not_occupied {
    ($color: expr, $board: expr) => {
        ($board).pieces[BISHOP_KINGS_SIDE_POSITIONS[($color) as usize]] == EMPTY_SQUARE
            && ($board).pieces[KNIGHTS_KINGS_SIDE_POSITIONS[($color) as usize]] == EMPTY_SQUARE
    };
}

pub fn generate_pseudo_legal_king_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let king = board.pieces[from];
    let king_color = get_piece_color(king);
    let mut to = signed_from + 7;
    if to < 64 && to & 7 < from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    to = signed_from + 8;
    if to < 64 && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    to = signed_from + 9;
    if to < 64 && to & 7 > from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    to = signed_from + 1;
    if to & 7 > from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    to = signed_from - 7;
    if to >= 0 && to & 7 > from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    to = signed_from - 8;
    if to >= 0 && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    to = signed_from - 9;
    if to >= 0 && to & 7 < from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    to = signed_from - 1;
    if to & 7 < from_file && board.can_be_moved(to as usize, king_color) {
        result.push(new_move(from, to as usize, board));
    }

    if board.has_king_stayed_in_place[king_color as usize] {
        if squares_between_king_and_queens_rook_are_not_occupied!(king_color, board)
            && board.has_queens_rook_stayed_in_place[king_color as usize]
        {
            result.push(new_castling(CASTLING_QUEENS_SIDE, from, king, king_color))
        } else if squares_between_king_and_kings_rook_are_not_occupied!(king_color, board)
            && board.has_kings_rook_stayed_in_place[king_color as usize]
        {
            result.push(new_castling(CASTLING_KINGS_SIDE, from, king, king_color))
        }
    }
}
