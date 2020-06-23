extern crate rand;

use crate::board::Board;
use crate::moves::constructors::{new_castling, new_move};
use crate::moves::{Move, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::EMPTY_SQUARE;
use rand::rngs::ThreadRng;
use rand::Rng;

pub const MOVE_PSEUDO_LEGALITY_VALIDATORS: [fn(i8, i8, &Board, Color) -> bool; 8] = [
    |from_file, to, board, king_color| {
        to < 64 && to & 7 < from_file && board.can_be_moved(to, king_color)
    },
    |_, to, board, king_color| to < 64 && board.can_be_moved(to, king_color),
    |from_file, to, board, king_color| {
        to < 64 && to & 7 > from_file && board.can_be_moved(to, king_color)
    },
    |from_file, to, board, king_color| to & 7 > from_file && board.can_be_moved(to, king_color),
    |from_file, to, board, king_color| {
        to >= 0 && to & 7 > from_file && board.can_be_moved(to, king_color)
    },
    |_, to, board, king_color| to >= 0 && board.can_be_moved(to, king_color),
    |from_file, to, board, king_color| {
        to >= 0 && to & 7 < from_file && board.can_be_moved(to, king_color)
    },
    |from_file, to, board, king_color| to & 7 < from_file && board.can_be_moved(to, king_color),
];
pub const ATTACK_PSEUDO_LEGALITY_VALIDATORS: [fn(i8, i8) -> bool; 8] = [
    |from_file, to| to < 64 && to & 7 < from_file,
    |_, to| to < 64,
    |from_file, to| to < 64 && to & 7 > from_file,
    |from_file, to| to & 7 > from_file,
    |from_file, to| to >= 0 && to & 7 > from_file,
    |_, to| to >= 0,
    |from_file, to| to >= 0 && to & 7 < from_file,
    |from_file, to| to & 7 < from_file,
];
pub fn get_moves_to(from: usize) -> [i8; 8] {
    let signed_from = from as i8;
    [
        signed_from + 7,
        signed_from + 8,
        signed_from + 9,
        signed_from + 1,
        signed_from - 7,
        signed_from - 8,
        signed_from - 9,
        signed_from - 1,
    ]
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let king = board.state.pieces[from];
    let king_color = get_piece_color(king);
    get_moves_to(from)
        .iter()
        .enumerate()
        .filter(|(index, to)| {
            MOVE_PSEUDO_LEGALITY_VALIDATORS[*index](from_file, **to, board, king_color)
        })
        .for_each(|(_, to)| result.push(new_move(from, *to, board)));

    if board.is_castling_queens_side_pseudo_legal(king_color) {
        result.push(new_castling(CASTLING_QUEENS_SIDE, from, king, king_color))
    }

    if board.is_castling_kings_side_pseudo_legal(king_color) {
        result.push(new_castling(CASTLING_KINGS_SIDE, from, king, king_color))
    }
}

#[inline]
pub fn generate_random_getting_out_of_check_move(
    from: usize,
    legal_moves_to: &Vec<i8>,
    board: &Board,
    rng: &mut ThreadRng,
) -> Move {
    let index = rng.gen_range(0, legal_moves_to.len());
    new_move(from, legal_moves_to[index], board)
}

pub fn generate_random_legal_move(
    from: usize,
    board: &mut Board,
    rng: &mut ThreadRng,
) -> Option<Move> {
    let king = board.state.pieces[from];
    let king_color = get_piece_color(king);

    if rng.gen_bool(0.5) {
        if rng.gen_bool(0.5) && board.is_castling_queens_side_pseudo_legal(king_color) {
            return Some(new_castling(CASTLING_QUEENS_SIDE, from, king, king_color));
        } else if board.is_castling_kings_side_pseudo_legal(king_color) {
            return Some(new_castling(CASTLING_KINGS_SIDE, from, king, king_color));
        }
    }

    let legal_moves_to = get_legal_moves_to(from, board);
    return if legal_moves_to.is_empty() {
        None
    } else {
        Some(generate_random_getting_out_of_check_move(
            from,
            &legal_moves_to,
            board,
            rng,
        ))
    };
}

pub fn get_legal_moves_to(from: usize, board: &mut Board) -> Vec<i8> {
    let king = board.state.pieces[from];
    board.state.pieces[from] = EMPTY_SQUARE;
    let from_file = from as i8 & 7;
    let result = get_moves_to(from)
        .iter()
        .enumerate()
        .filter(|&(i, &to)| {
            MOVE_PSEUDO_LEGALITY_VALIDATORS[i](from_file, to, board, board.state.side)
                && !board.is_square_attacked(to, board.state.side)
        })
        .map(|(_, &to)| to)
        .collect();
    board.state.pieces[from] = king;
    result
}

pub fn can_be_moved(from: usize, board: &mut Board) -> bool {
    let king = board.state.pieces[from];
    board.state.pieces[from] = EMPTY_SQUARE;
    let from_file = from as i8 & 7;
    let result = get_moves_to(from).iter().enumerate().any(|(i, to)| {
        MOVE_PSEUDO_LEGALITY_VALIDATORS[i](from_file, *to, board, board.state.side)
            && !board.is_square_attacked(*to, board.state.side)
    });
    board.state.pieces[from] = king;
    result
}
