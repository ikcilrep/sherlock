use crate::board::Board;
use crate::moves::Move;
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::sliders::add_sliding_move;

fn generate_pseudo_legal_moves_on_northeast(
    from: usize,
    from_file: i8,
    bishop_color: Color,
    board: &Board,
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
    bishop_color: Color,
    board: &Board,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 - 9;
    while to > 0 && to & 7 > from_file && add_sliding_move(from, to, bishop_color, result, board) {
        to -= 9;
    }
}

fn generate_pseudo_legal_moves_on_northwest(
    from: usize,
    from_file: i8,
    bishop_color: Color,
    board: &Board,
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
    bishop_color: Color,
    board: &Board,
    result: &mut Vec<Move>,
) {
    let mut to = from as i8 - 7;
    while to > 0 && to & 7 < from_file && add_sliding_move(from, to, bishop_color, result, board) {
        to -= 7;
    }
}

pub fn generate_pseudo_legal_moves(from: usize, board: &Board, result: &mut Vec<Move>) {
    let bishop_color = get_piece_color(board.pieces[from]);
    let signed_from = from as i8;
    let from_file = signed_from & 7;
    generate_pseudo_legal_moves_on_northeast(from, from_file, bishop_color, board, result);
    generate_pseudo_legal_moves_on_southwest(from, from_file, bishop_color, board, result);
    generate_pseudo_legal_moves_on_northwest(from, from_file, bishop_color, board, result);
    generate_pseudo_legal_moves_on_southeast(from, from_file, bishop_color, board, result);
}
