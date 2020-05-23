extern crate rand;

use crate::board::Board;
use crate::moves::constructors::{new_en_passant, new_move, new_promotion};
use crate::moves::{Move, NULL_MOVE};
use crate::pieces::color::{colorize_piece, get_piece_color, uncolorize_piece, Color};
use crate::pieces::{BISHOP, EMPTY_SQUARE, KNIGHT, PAWN, QUEEN, ROOK};
use rand::rngs::ThreadRng;
use rand::Rng;

const PAWN_STEPS: [[i8; 3]; 2] = [[7, 8, 9], [-9, -8, -7]];
const PAWN_START_ROWS: [usize; 2] = [1, 6];

const NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS: [fn(i8, i8, &Board, Color) -> bool; 3] = [
    |from_file, to, board, pawn_color| {
        to < 64
            && to >= 0
            && to & 7 < from_file
            && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
    },
    |_, to, board, _| to < 64 && to >= 0 && board.pieces[to as usize] == EMPTY_SQUARE,
    |from_file, to, board, pawn_color| {
        to < 64
            && to >= 0
            && to & 7 > from_file
            && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
    },
];

fn get_nearest_moves_to(from: usize, pawn_color: Color) -> [i8; 3] {
    let signed_from = from as i8;
    [
        signed_from + PAWN_STEPS[pawn_color as usize][0],
        signed_from + PAWN_STEPS[pawn_color as usize][1],
        signed_from + PAWN_STEPS[pawn_color as usize][2],
    ]
}

fn add_promotions(from: usize, to: i8, pawn_color: Color, board: &Board, result: &mut Vec<Move>) {
    let colorized_bishop = colorize_piece(BISHOP, pawn_color);
    let colorized_knight = colorize_piece(KNIGHT, pawn_color);
    let colorized_queen = colorize_piece(QUEEN, pawn_color);
    let colorized_rook = colorize_piece(ROOK, pawn_color);
    result.push(new_promotion(from, to, colorized_bishop, board));
    result.push(new_promotion(from, to, colorized_knight, board));
    result.push(new_promotion(from, to, colorized_queen, board));
    result.push(new_promotion(from, to, colorized_rook, board));
}

fn random_promotion(
    from: usize,
    to: i8,
    pawn_color: Color,
    board: &Board,
    rng: &mut ThreadRng,
) -> Move {
    if rng.gen_bool(0.5) {
        new_promotion(from, to, colorize_piece(QUEEN, pawn_color), board)
    } else if rng.gen_bool(0.25) {
        new_promotion(from, to, colorize_piece(ROOK, pawn_color), board)
    } else if rng.gen_bool(0.125) {
        new_promotion(from, to, colorize_piece(BISHOP, pawn_color), board)
    } else {
        new_promotion(from, to, colorize_piece(KNIGHT, pawn_color), board)
    }
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let pawn_color = get_piece_color(board.pieces[from]);
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let mut to = signed_from + PAWN_STEPS[pawn_color as usize][0];

    if to & 7 < from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
    {
        if to < 56 && to > 7 {
            result.push(new_move(from, to, board));
        } else if to < 64 && to >= 0 {
            add_promotions(from, to, pawn_color, board, result);
        }
    }

    to = signed_from + PAWN_STEPS[pawn_color as usize][1];
    let from_row = from >> 3;
    if to < 56 && to > 7 && board.pieces[to as usize] == EMPTY_SQUARE {
        result.push(new_move(from, to, board));

        to += PAWN_STEPS[pawn_color as usize][1];
        if from_row == PAWN_START_ROWS[pawn_color as usize]
            && board.pieces[to as usize] == EMPTY_SQUARE
        {
            result.push(new_move(from, to, board));
        }
    } else if to < 64 && to >= 0 && board.pieces[to as usize] == EMPTY_SQUARE {
        add_promotions(from, to, pawn_color, board, result);
    }

    to = signed_from + PAWN_STEPS[pawn_color as usize][2];
    if to & 7 > from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
    {
        if to < 56 && to > 7 {
            result.push(new_en_passant(from, to, board));
        } else if to < 64 && to >= 0 {
            add_promotions(from, to, pawn_color, board, result);
        }
    }
}

#[inline]
fn is_move_northwest_pseudo_legal(from_file: i8, to: i8, board: &Board, pawn_color: Color) -> bool {
    to < 64
        && to >= 0
        && to & 7 < from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
}

#[inline]
fn is_move_north_pseudo_legal(_: i8, to: i8, board: &Board, _: Color) -> bool {
    to < 64 && to >= 0 && board.pieces[to as usize] == EMPTY_SQUARE
}

#[inline]
fn is_move_northeast_pseudo_legal(from_file: i8, to: i8, board: &Board, pawn_color: Color) -> bool {
    to < 64
        && to >= 0
        && to & 7 > from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
}

#[inline]
fn get_move_north(from: usize, to: i8, board: &Board, pawn_color: Color) -> Move {
    let from_row = from >> 3;
    let new_to = to + PAWN_STEPS[pawn_color as usize][1];

    return if from_row == PAWN_START_ROWS[pawn_color as usize]
        && board.pieces[new_to as usize] == EMPTY_SQUARE
    {
        new_move(from, new_to, board)
    } else {
        new_move(from, to, board)
    };
}

#[inline]
fn get_capture(from: usize, to: i8, board: &Board, _: Color) -> Move {
    new_move(from, to, board)
}

pub fn generate_random_pseudo_legal_move(from: usize, board: &Board, rng: &mut ThreadRng) -> Move {
    let pawn_color = get_piece_color(board.pieces[from]);
    let signed_from = from as i8;
    let from_file = signed_from & 7;

    let move_pseudo_legality_validators = [
        is_move_northwest_pseudo_legal,
        is_move_north_pseudo_legal,
        is_move_northeast_pseudo_legal,
    ];

    let moves_to = [
        signed_from + PAWN_STEPS[pawn_color as usize][0],
        signed_from + PAWN_STEPS[pawn_color as usize][1],
        signed_from + PAWN_STEPS[pawn_color as usize][2],
    ];

    let move_getters = [get_capture, get_move_north, get_capture];

    let start = rng.gen_range(0, 3);
    let mut i = start;
    while {
        if move_pseudo_legality_validators[i](from_file, moves_to[i], board, pawn_color) {
            return if moves_to[i] > 7 && moves_to[i] < 56 {
                move_getters[i](from, moves_to[i], board, pawn_color)
            } else {
                random_promotion(from, moves_to[i], pawn_color, board, rng)
            };
        }
        i += 1;
        i %= 3;
        i != start
    } {}
    NULL_MOVE
}

#[inline]
pub fn can_be_moved(from: usize, board: &mut Board) -> bool {
    let from_file = from as i8 & 7;
    get_nearest_moves_to(from, board.state.side)
        .iter()
        .enumerate()
        .any(|(i, to)| {
            NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS[i](from_file, *to, board, board.state.side)
        })
}

pub fn can_be_moved_to_without_capture(to: i8, board: &mut Board) -> bool {
    let color = board.state.side as usize;
    let from = to - PAWN_STEPS[color][1];
    uncolorize_piece(board.pieces[from as usize]) == PAWN
        && !board.is_piece_pinned(from, to, board.state.king_positions[color])
}
