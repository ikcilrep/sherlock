use crate::board::moves;
use crate::board::moves::Move;
use crate::board::pieces::color::Color;
use crate::board::pieces::EMPTY_SQUARE;
use crate::board::Board;

pub fn add_sliding_move(
    from: usize,
    to: usize,
    slider_color: Color,
    result: &mut Vec<Move>,
    board: &Board,
) -> bool {
    result.push(new_move!(from, to, board));
    if board.pieces[to] != EMPTY_SQUARE {
        if color!(board.pieces[to]) == slider_color {
            result.pop();
        }
        return false;
    }
    true
}
