extern crate rand;

use crate::board::Board;
use crate::moves::{get_to, Move};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::sliders::add_sliding_move;
use rand::rngs::ThreadRng;
use rand::Rng;

fn get_nearest_moves_to(from: usize) -> [i8; 4] {
    let signed_from = from as i8;
    [
        signed_from + 1,
        signed_from - 1,
        signed_from + 8,
        signed_from - 8,
    ]
}

const NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS: [fn(i8, &Board, Color) -> bool; 4] = [
    |to, board, rook_color| to & 7 != 0 && board.can_be_moved(to, rook_color),
    |to, board, rook_color| to & 7 != 7 && board.can_be_moved(to, rook_color),
    |to, board, rook_color| to < 64 && board.can_be_moved(to, rook_color),
    |to, board, rook_color| to >= 0 && board.can_be_moved(to, rook_color),
];

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
    let rook_color = get_piece_color(board.state.pieces[from]);
    generate_pseudo_legal_moves_on_east(from, board, rook_color, result);
    generate_pseudo_legal_moves_on_west(from, board, rook_color, result);
    generate_pseudo_legal_moves_on_north(from, board, rook_color, result);
    generate_pseudo_legal_moves_on_south(from, board, rook_color, result);
}

pub fn generate_random_legal_move(
    from: usize,
    board: &mut Board,
    rng: &mut ThreadRng,
) -> Option<Move> {
    let move_generators = [
        generate_pseudo_legal_moves_on_east,
        generate_pseudo_legal_moves_on_west,
        generate_pseudo_legal_moves_on_north,
        generate_pseudo_legal_moves_on_south,
    ];

    let start = rng.gen_range(0, 4);
    let rook_color = get_piece_color(board.state.pieces[from]);
    let king_location = board.state.king_positions[rook_color as usize];
    let signed_from = from as i8;
    let mut i = start;
    while {
        let mut moves = Vec::new();
        move_generators[i](from, board, rook_color, &mut moves);
        if moves.is_empty() {
            i += 1;
            i &= 3;
        } else {
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
        i != start
    } {}
    None
}

#[inline]
pub fn can_be_moved(from: usize, board: &mut Board) -> bool {
    get_nearest_moves_to(from)
        .iter()
        .enumerate()
        .any(|(i, to)| NEAREST_MOVES_PSEUDO_LEGALITY_VALIDATORS[i](*to, board, board.state.side))
}
