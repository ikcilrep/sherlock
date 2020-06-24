extern crate rand;

use crate::board::Board;
use crate::moves::{get_to, Move};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::sliders::add_sliding_move;
use rand::rngs::ThreadRng;
use rand::Rng;

const NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS: [fn(i8, i8, &Board, Color) -> bool; 4] = [
    |to, from_file, board, bishop_color| {
        to < 64 && to & 7 > from_file && board.can_be_moved(to, bishop_color)
    },
    |to, from_file, board, bishop_color| {
        to >= 0 && to & 7 < from_file && board.can_be_moved(to, bishop_color)
    },
    |to, from_file, board, bishop_color| {
        to < 64 && to & 7 < from_file && board.can_be_moved(to, bishop_color)
    },
    |to, from_file, board, bishop_color| {
        to >= 0 && to & 7 > from_file && board.can_be_moved(to, bishop_color)
    },
];

fn get_nearest_moves_to(from: usize) -> [i8; 4] {
    let signed_from = from as i8;
    [
        signed_from + 9,
        signed_from - 9,
        signed_from + 7,
        signed_from - 7,
    ]
}

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
    while to >= 0 && to & 7 < from_file && add_sliding_move(from, to, bishop_color, result, board) {
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
    while to >= 0 && to & 7 > from_file && add_sliding_move(from, to, bishop_color, result, board) {
        to -= 7;
    }
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let bishop_color = get_piece_color(board.state.pieces[from]);
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    generate_pseudo_legal_moves_on_northeast(from, from_file, board, bishop_color, result);
    generate_pseudo_legal_moves_on_southwest(from, from_file, board, bishop_color, result);
    generate_pseudo_legal_moves_on_northwest(from, from_file, board, bishop_color, result);
    generate_pseudo_legal_moves_on_southeast(from, from_file, board, bishop_color, result);
}

pub fn generate_random_legal_move(
    from: usize,
    board: &mut Board,
    rng: &mut ThreadRng,
) -> Option<Move> {
    let move_generators = [
        generate_pseudo_legal_moves_on_northeast,
        generate_pseudo_legal_moves_on_southwest,
        generate_pseudo_legal_moves_on_northwest,
        generate_pseudo_legal_moves_on_southeast,
    ];

    let start = rng.gen_range(0, 4);
    let bishop_color = get_piece_color(board.state.pieces[from]);
    let king_location = board.state.king_positions[bishop_color as usize];
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let mut i = start;
    let mut moves = Vec::new();

    while {
        move_generators[i](from, from_file, board, bishop_color, &mut moves);
        if !moves.is_empty() {
            let move_start = rng.gen_range(0, moves.len());
            let mut j = move_start;
            while {
                if !board.is_piece_pinned(signed_from, get_to(moves[j]) as i8, king_location) {
                    return Some(moves[j]);
                }
                j += 1;
                j %= moves.len();
                j != move_start
            } {}
        }

        i += 1;
        i &= 3;
        i != start
    } {}
    None
}

#[inline]
pub fn can_be_moved(from: usize, board: &mut Board) -> bool {
    let from_file = from as i8 & 7;
    get_nearest_moves_to(from)
        .iter()
        .enumerate()
        .any(|(i, to)| {
            NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS[i](*to, from_file, board, board.state.side)
        })
}
