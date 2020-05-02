extern crate rand;

use crate::board::Board;
use crate::moves::constructors::{new_en_passant, new_move, new_promotion};
use crate::moves::Move;
use crate::pieces::color::{colorize_piece, get_piece_color, Color};
use crate::pieces::{BISHOP, KNIGHT, QUEEN, ROOK};
use rand::rngs::ThreadRng;
use rand::Rng;

const PAWN_STEPS: [[i8; 3]; 2] = [[7, 8, 9], [-9, -8, -7]];
const PAWN_START_ROWS: [usize; 2] = [1, 6];

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

    if to & 7 < from_file && (board.en_passant_square == to || board.can_capture(to, pawn_color)) {
        if to < 56 && to > 8 {
            result.push(new_move(from, to, board));
        } else if to < 64 && to > 0 {
            add_promotions(from, to, pawn_color, board, result);
        }
    }

    to = signed_from + PAWN_STEPS[pawn_color as usize][1];
    let from_row = from >> 3;
    if to < 56 && to > 8 {
        result.push(new_move(from, to, board));
        if from_row == PAWN_START_ROWS[pawn_color as usize] {
            to += PAWN_STEPS[pawn_color as usize][1];
            result.push(new_move(from, to, board));
        }
    } else if to < 64 && to > 0 {
        add_promotions(from, to, pawn_color, board, result);
    }

    to = signed_from + PAWN_STEPS[pawn_color as usize][2];
    if to & 7 > from_file && (board.en_passant_square == to || board.can_capture(to, pawn_color)) {
        if to < 56 && to > 8 {
            result.push(new_en_passant(from, to, board));
        } else if to < 64 && to > 0 {
            add_promotions(from, to, pawn_color, board, result);
        }
    }
}
