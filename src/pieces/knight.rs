extern crate rand;

use crate::board::Board;
use crate::moves::constructors::new_move;
use crate::moves::{Move, NULL_MOVE};
use crate::pieces::color::{get_piece_color, Color};
use rand::rngs::ThreadRng;
use rand::Rng;

pub const MOVE_PSEUDO_LEGALITY_VALIDATORS: [fn(i8, i8, &Board, Color) -> bool; 8] = [
    is_move_northeast_pseudo_legal,
    is_move_northwest_pseudo_legal,
    is_move_southeast_pseudo_legal,
    is_move_southwest_pseudo_legal,
    is_move_northeast_pseudo_legal,
    is_move_northwest_pseudo_legal,
    is_move_southeast_pseudo_legal,
    is_move_southwest_pseudo_legal,
];

pub fn get_moves_to(from: usize) -> [i8; 8] {
    let signed_from = from as i8;
    [
        signed_from + 17,
        signed_from + 15,
        signed_from - 17,
        signed_from - 15,
        signed_from + 10,
        signed_from + 6,
        signed_from - 10,
        signed_from - 6,
    ]
}

#[inline]
pub fn is_move_northeast_pseudo_legal(
    from_file: i8,
    to: i8,
    board: &Board,
    knight_color: Color,
) -> bool {
    to < 64 && to & 7 > from_file && board.can_be_moved(to, knight_color)
}

#[inline]
pub fn is_move_northwest_pseudo_legal(
    from_file: i8,
    to: i8,
    board: &Board,
    knight_color: Color,
) -> bool {
    to < 64 && to & 7 < from_file && board.can_be_moved(to, knight_color)
}

#[inline]
pub fn is_move_southeast_pseudo_legal(
    from_file: i8,
    to: i8,
    board: &Board,
    knight_color: Color,
) -> bool {
    to >= 0 && to & 7 > from_file && board.can_be_moved(to, knight_color)
}

#[inline]
pub fn is_move_southwest_pseudo_legal(
    from_file: i8,
    to: i8,
    board: &Board,
    knight_color: Color,
) -> bool {
    to >= 0 && to & 7 < from_file && board.can_be_moved(to, knight_color)
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let knight_color = get_piece_color(board.pieces[from]);

    get_moves_to(from)
        .iter()
        .enumerate()
        .filter(|(index, to)| {
            MOVE_PSEUDO_LEGALITY_VALIDATORS[*index](from_file, **to, board, knight_color)
        })
        .for_each(|(_, to)| result.push(new_move(from, *to, board)));
}

pub fn generate_random_pseudo_legal_move(from: usize, board: &Board, rng: &mut ThreadRng) -> Move {
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    let knight_color = get_piece_color(board.pieces[from]);
    let moves_to = get_moves_to(from);

    let start = rng.gen_range(0, 8);
    let mut i = start;
    while {
        if MOVE_PSEUDO_LEGALITY_VALIDATORS[i](from_file, moves_to[i], board, knight_color) {
            return new_move(from, moves_to[i], board);
        }
        i += 1;
        i &= 7;
        i != start
    } {}
    NULL_MOVE
}
