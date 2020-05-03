extern crate rand;

use crate::board::Board;
use crate::moves::constructors::{new_castling, new_move};
use crate::moves::{Move, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, NULL_MOVE};
use crate::pieces::color::{get_piece_color, Color};
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::Rng;

#[inline]
fn is_move_northwest_pseudo_legal(from_file: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to < 64 && to & 7 < from_file && board.can_be_moved(to, king_color)
}

#[inline]
fn is_move_north_pseudo_legal(_: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to < 64 && board.can_be_moved(to, king_color)
}

#[inline]
fn is_move_northeast_pseudo_legal(from_file: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to < 64 && to & 7 > from_file && board.can_be_moved(to, king_color)
}

#[inline]
fn is_move_east_pseudo_legal(from_file: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to & 7 > from_file && board.can_be_moved(to, king_color)
}

#[inline]
fn is_move_southeast_pseudo_legal(from_file: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to >= 0 && to & 7 > from_file && board.can_be_moved(to, king_color)
}

#[inline]
fn is_move_south_pseudo_legal(_: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to >= 0 && board.can_be_moved(to, king_color)
}

#[inline]
fn is_move_southwest_pseudo_legal(from_file: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to >= 0 && to & 7 < from_file && board.can_be_moved(to, king_color)
}

#[inline]
fn is_move_west_pseudo_legal(from_file: i8, to: i8, board: &Board, king_color: Color) -> bool {
    to & 7 < from_file && board.can_be_moved(to, king_color)
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let king = board.pieces[from];
    let king_color = get_piece_color(king);
    let mut to = signed_from + 7;
    if is_move_northwest_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board))
    }

    to = signed_from + 8;
    if is_move_north_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 9;
    if is_move_northeast_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from + 1;
    if is_move_east_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 7;
    if is_move_southeast_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 8;
    if is_move_south_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 9;
    if is_move_southwest_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board));
    }

    to = signed_from - 1;
    if is_move_west_pseudo_legal(from_file, to, board, king_color) {
        result.push(new_move(from, to, board));
    }

    if board.is_castling_queens_side_pseudo_legal(king_color) {
        result.push(new_castling(CASTLING_QUEENS_SIDE, from, king, king_color))
    }

    if board.is_castling_kings_side_pseudo_legal(king_color) {
        result.push(new_castling(CASTLING_KINGS_SIDE, from, king, king_color))
    }
}

pub fn generate_random_pseudo_legal_move(from: usize, board: &Board, rng: &mut ThreadRng) -> Move {
    let move_pseudo_legality_validators = [
        is_move_northwest_pseudo_legal,
        is_move_north_pseudo_legal,
        is_move_northeast_pseudo_legal,
        is_move_east_pseudo_legal,
        is_move_southeast_pseudo_legal,
        is_move_south_pseudo_legal,
        is_move_southwest_pseudo_legal,
        is_move_west_pseudo_legal,
    ];
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let king = board.pieces[from];
    let king_color = get_piece_color(king);

    let moves_to = [
        signed_from + 7,
        signed_from + 8,
        signed_from + 9,
        signed_from + 1,
        signed_from - 7,
        signed_from - 8,
        signed_from - 9,
        signed_from - 1,
    ];

    if rng.gen_bool(0.5) {
        if rng.gen_bool(0.5) && board.is_castling_queens_side_pseudo_legal(king_color) {
            return new_castling(CASTLING_QUEENS_SIDE, from, king, king_color);
        } else if board.is_castling_kings_side_pseudo_legal(king_color) {
            return new_castling(CASTLING_KINGS_SIDE, from, king, king_color);
        }
    }

    let start = rng.gen_range(0, 8);
    let mut i = start;
    while {
        if move_pseudo_legality_validators[i](from_file, moves_to[i], board, king_color) {
            return new_move(from, moves_to[i], board);
        }
        i += 1;
        i &= 7;
        i != start
    } {}
    NULL_MOVE
}
