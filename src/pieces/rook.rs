extern crate rand;

use crate::board::Board;
use crate::moves::{Move, NULL_MOVE};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::sliders::add_sliding_move;
use rand::rngs::ThreadRng;
use rand::Rng;

#[inline]
fn generate_pseudo_legal_moves_on_east(
    from: usize,
    board: &Board,
    rook_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 + 1;
    while to & 7 != 0 && add_sliding_move(from, to, rook_color, result, board) {
        to += 1;
    }
}

#[inline]
fn generate_pseudo_legal_moves_on_west(
    from: usize,
    board: &Board,
    rook_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 - 1;
    while to & 7 != 7 && add_sliding_move(from, to, rook_color, result, board) {
        to -= 1;
    }
}

#[inline]
fn generate_pseudo_legal_moves_on_north(
    from: usize,
    board: &Board,
    rook_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 + 8;
    while to < 64 && add_sliding_move(from, to, rook_color, result, board) {
        to += 8;
    }
}

#[inline]
fn generate_pseudo_legal_moves_on_south(
    from: usize,
    board: &Board,
    rook_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 - 8;
    while to >= 0 && add_sliding_move(from, to, rook_color, result, board) {
        to -= 8;
    }
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let rook_color = get_piece_color(board.pieces[from]);
    generate_pseudo_legal_moves_on_east(from, board, rook_color, result);
    generate_pseudo_legal_moves_on_west(from, board, rook_color, result);
    generate_pseudo_legal_moves_on_north(from, board, rook_color, result);
    generate_pseudo_legal_moves_on_south(from, board, rook_color, result);
}

pub fn generate_random_pseudo_legal_move(from: usize, board: &Board, rng: &mut ThreadRng) -> Move {
    let move_generators = [
        generate_pseudo_legal_moves_on_east,
        generate_pseudo_legal_moves_on_west,
        generate_pseudo_legal_moves_on_north,
        generate_pseudo_legal_moves_on_south,
    ];

    let start = rng.gen_range(0, 4);
    let rook_color = get_piece_color(board.pieces[from]);
    let mut i = start;
    while {
        let mut moves = Vec::new();
        move_generators[i](from, board, rook_color, &mut moves);
        if moves.is_empty() {
            i += 1;
            i &= 3;
        } else {
            let move_index = rng.gen_range(0, moves.len());
            return moves[move_index];
        }
        i != start
    } {}
    NULL_MOVE
}
