extern crate rand;

use crate::board::Board;
use crate::moves::{Move, NULL_MOVE};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::sliders::add_sliding_move;
use rand::rngs::ThreadRng;
use rand::Rng;

fn generate_pseudo_legal_moves_on_northeast(
    from: usize,
    from_file: i8,
    board: &Board,
    bishop_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 + 9;

    while to < 64 && to & 7 > from_file && add_sliding_move(from, to, bishop_color, result, board) {
        to += 9;
    }
}

fn generate_pseudo_legal_moves_on_southwest(
    from: usize,
    from_file: i8,
    board: &Board,
    bishop_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 - 9;
    while to > 0 && to & 7 < from_file && add_sliding_move(from, to, bishop_color, result, board) {
        to -= 9;
    }
}

fn generate_pseudo_legal_moves_on_northwest(
    from: usize,
    from_file: i8,
    board: &Board,
    bishop_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 + 7;
    while to < 64 && to & 7 < from_file && add_sliding_move(from, to, bishop_color, result, board) {
        to += 7;
    }
}

fn generate_pseudo_legal_moves_on_southeast(
    from: usize,
    from_file: i8,
    board: &Board,
    bishop_color: Color,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 - 7;
    while to > 0 && to & 7 > from_file && add_sliding_move(from, to, bishop_color, result, board) {
        to -= 7;
    }
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let bishop_color = get_piece_color(board.pieces[from]);
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    generate_pseudo_legal_moves_on_northeast(from, from_file, board, bishop_color, result);
    generate_pseudo_legal_moves_on_southwest(from, from_file, board, bishop_color, result);
    generate_pseudo_legal_moves_on_northwest(from, from_file, board, bishop_color, result);
    generate_pseudo_legal_moves_on_southeast(from, from_file, board, bishop_color, result);
}

pub fn generate_random_pseudo_legal_move(from: usize, board: &Board, rng: &mut ThreadRng) -> Move {
    let move_generators = [
        generate_pseudo_legal_moves_on_northeast,
        generate_pseudo_legal_moves_on_southwest,
        generate_pseudo_legal_moves_on_northwest,
        generate_pseudo_legal_moves_on_southeast,
    ];

    let start = rng.gen_range(0, 4);
    let rook_color = get_piece_color(board.pieces[from]);
    let from_file = from as i8 & 7;
    let mut i = start;
    while {
        let mut moves = Vec::new();
        move_generators[i](from, from_file, board, rook_color, &mut moves);
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
