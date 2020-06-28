extern crate rand;

use crate::board::Board;
use crate::moves::constructors::{new_en_passant, new_move, new_promotion};
use crate::moves::Move;
use crate::pieces::color::{colorize_piece, get_piece_color, Color};
use crate::pieces::ColorizedPiece;
use crate::pieces::{BISHOP, EMPTY_SQUARE, KNIGHT, PAWN, QUEEN, ROOK};
use rand::rngs::ThreadRng;
use rand::Rng;

pub const PAWN_STEPS: [[i8; 3]; 2] = [[7, 8, 9], [-9, -8, -7]];
pub const PAWN_START_RANKS: [usize; 2] = [1, 6];

const NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS: [fn(i8, i8, &Board, Color) -> bool; 3] = [
    |from_file, to, board, pawn_color| {
        board.is_square_on_board(to)
            && to & 7 < from_file
            && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
    },
    |_, to, board, _| board.state.pieces[to as usize] == EMPTY_SQUARE,
    |from_file, to, board, pawn_color| {
        board.is_square_on_board(to)
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

pub fn random_promotion(
    from: usize,
    to: i8,
    pawn_color: Color,
    board: &Board,
    rng: &mut ThreadRng,
) -> Move {
    if rng.gen_bool(0.25) {
        new_promotion(from, to, colorize_piece(QUEEN, pawn_color), board)
    } else if rng.gen_bool(0.25) {
        new_promotion(from, to, colorize_piece(ROOK, pawn_color), board)
    } else if rng.gen_bool(0.25) {
        new_promotion(from, to, colorize_piece(BISHOP, pawn_color), board)
    } else {
        new_promotion(from, to, colorize_piece(KNIGHT, pawn_color), board)
    }
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let pawn_color = get_piece_color(board.state.pieces[from]);
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let mut to = signed_from + PAWN_STEPS[pawn_color as usize][0];

    if to & 7 < from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
    {
        if to < 56 && to > 7 {
            result.push(new_move(from, to, board));
        } else if board.is_square_on_board(to) {
            add_promotions(from, to, pawn_color, board, result);
        }
    }

    to = signed_from + PAWN_STEPS[pawn_color as usize][1];
    let from_row = from >> 3;
    if to < 56 && to > 7 && board.state.pieces[to as usize] == EMPTY_SQUARE {
        result.push(new_move(from, to, board));

        to += PAWN_STEPS[pawn_color as usize][1];
        if from_row == PAWN_START_RANKS[pawn_color as usize]
            && board.state.pieces[to as usize] == EMPTY_SQUARE
        {
            result.push(new_move(from, to, board));
        }
    } else if board.is_square_on_board(to) && board.state.pieces[to as usize] == EMPTY_SQUARE {
        add_promotions(from, to, pawn_color, board, result);
    }

    to = signed_from + PAWN_STEPS[pawn_color as usize][2];
    if to & 7 > from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
    {
        if to < 56 && to > 7 {
            result.push(new_en_passant(from, to, pawn_color, board));
        } else if board.is_square_on_board(to) {
            add_promotions(from, to, pawn_color, board, result);
        }
    }
}

#[inline]
fn is_move_northwest_pseudo_legal(from_file: i8, to: i8, board: &Board, pawn_color: Color) -> bool {
    board.is_square_on_board(to)
        && to & 7 < from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
}

#[inline]
fn is_move_north_pseudo_legal(_: i8, to: i8, board: &Board, _: Color) -> bool {
    board.is_square_on_board(to) && board.state.pieces[to as usize] == EMPTY_SQUARE
}

#[inline]
fn is_move_northeast_pseudo_legal(from_file: i8, to: i8, board: &Board, pawn_color: Color) -> bool {
    board.is_square_on_board(to)
        && to & 7 > from_file
        && (board.state.en_passant_square == to || board.can_capture(to, pawn_color))
}

fn get_random_near_move_north(from: usize, to: i8, board: &Board, rng: &mut ThreadRng) -> Move {
    if to > 7 && to < 56 {
        new_move(from, to, board)
    } else {
        random_promotion(from, to, board.state.side, board, rng)
    }
}

#[inline]
fn get_random_move_north(
    from: usize,
    to: i8,
    board: &Board,
    pawn_color: Color,
    rng: &mut ThreadRng,
) -> Move {
    let from_row = from >> 3;
    let new_to = to + PAWN_STEPS[pawn_color as usize][1];

    return if rng.gen_bool(0.5) {
        get_random_near_move_north(from, to, board, rng)
    } else if from_row == PAWN_START_RANKS[pawn_color as usize]
        && board.state.pieces[new_to as usize] == EMPTY_SQUARE
    {
        new_move(from, new_to, board)
    } else {
        get_random_near_move_north(from, to, board, rng)
    };
}

#[inline]
fn get_capture(from: usize, to: i8, board: &Board, _: Color, _: &mut ThreadRng) -> Move {
    new_move(from, to, board)
}

pub fn generate_random_legal_move(
    from: usize,
    board: &mut Board,
    rng: &mut ThreadRng,
) -> Option<Move> {
    let pawn_color = get_piece_color(board.state.pieces[from]);
    let king_location = board.state.king_positions[pawn_color as usize];
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

    let move_getters = [get_capture, get_random_move_north, get_capture];

    let start = rng.gen_range(0, 3);
    let mut i = start;
    while {
        if move_pseudo_legality_validators[i](from_file, moves_to[i], board, pawn_color)
            && !board.is_piece_pinned(signed_from, moves_to[i], king_location)
        {
            return Some(if moves_to[i] > 7 && moves_to[i] < 56 {
                move_getters[i](from, moves_to[i], board, pawn_color, rng)
            } else {
                random_promotion(from, moves_to[i], pawn_color, board, rng)
            });
        }
        i += 1;
        i %= 3;
        i != start
    } {}
    None
}

#[inline]
pub fn can_be_moved(from: usize, board: &mut Board) -> bool {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let king_location = board.state.king_positions[board.state.side as usize];

    get_nearest_moves_to(from, board.state.side)
        .iter()
        .enumerate()
        .any(|(i, &to)| {
            NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS[i](from_file, to, board, board.state.side)
                && !board.is_piece_pinned(signed_from, to, king_location)
        })
}

pub fn can_be_moved_on_empty_square_without_capture(empty_square: i8, board: &mut Board) -> bool {
    let color = board.state.side as usize;
    let colorized_pawn = colorize_piece(PAWN, board.state.side);

    let from1 = empty_square - PAWN_STEPS[color][1];
    let from2 = from1 - PAWN_STEPS[color][1];
    !board.is_piece_pinned(from1, empty_square, board.state.king_positions[color])
        && (board.is_square_on_board(from1) && board.state.pieces[from1 as usize] == colorized_pawn)
        || (PAWN_START_RANKS[color] == (from2 >> 3) as usize
            && board.state.pieces[from1 as usize] == EMPTY_SQUARE
            && board.state.pieces[from2 as usize] == colorized_pawn)
}

pub fn can_capture_on_enemy_occupied_square(enemy_occupied_square: i8, board: &mut Board) -> bool {
    let color = board.state.side as usize;
    let from1 = enemy_occupied_square - PAWN_STEPS[color][0];
    let from2 = enemy_occupied_square - PAWN_STEPS[color][2];
    let enemy_occupied_square_file = enemy_occupied_square & 7;
    let colorized_pawn = colorize_piece(PAWN, board.state.side);
    let mut can_capture = |from: i8| -> bool {
        board.is_square_on_board(from)
            && board.state.pieces[from as usize] == colorized_pawn
            && !board.is_piece_pinned(
                from,
                enemy_occupied_square,
                board.state.king_positions[color],
            )
    };

    (from1 & 7 > enemy_occupied_square_file && can_capture(from1))
        || (from2 & 7 < enemy_occupied_square_file && can_capture(from2))
}

pub fn is_move_legal(
    from: i8,
    to: i8,
    piece_to_move: ColorizedPiece,
    piece_after_promotion: ColorizedPiece,
    board: &mut Board,
) -> bool {
    let color = board.state.side as usize;
    let pawn_step = PAWN_STEPS[color][1];
    let distance = (to - from).abs();
    let king_location = board.state.king_positions[color];
    board.is_square_on_board(from)
        && board.state.pieces[from as usize] == piece_to_move
        && ((distance >= 7 && distance <= 9)
            || (distance == 16
                && board.state.pieces[(to - pawn_step) as usize] == EMPTY_SQUARE
                && (from >> 3) == PAWN_START_RANKS[color] as i8))
        && (distance & 1 == 0) == (board.state.pieces[to as usize] == EMPTY_SQUARE)
        && (piece_to_move == piece_after_promotion) == (to >= 8 && to < 56)
        && !board.is_piece_pinned(from, to, king_location)
}

pub fn is_legal_en_passant(
    from: i8,
    to: i8,
    piece_to_move: ColorizedPiece,
    piece_after_promotion: ColorizedPiece,
    board: &mut Board,
) -> bool {
    let distance = (to - from).abs();
    let color = board.state.side as usize;
    let king_location = board.state.king_positions[color];
    board.state.en_passant_square == to
        && piece_to_move == piece_after_promotion
        && (distance == 7 || distance == 9)
        && !board.is_piece_pinned(from, to, king_location)
}
