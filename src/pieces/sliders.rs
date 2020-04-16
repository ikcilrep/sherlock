use crate::board::Board;
use crate::moves::{new_move, Move};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::EMPTY_SQUARE;

pub fn add_sliding_move(
    from: usize,
    to: i8,
    slider_color: Color,
    result: &mut Vec<Move>,
    board: &Board,
) -> bool {
    result.push(new_move(from, to, board));
    if board.pieces[to as usize] != EMPTY_SQUARE {
        if get_piece_color(board.pieces[to as usize]) == slider_color {
            result.pop();
        }
        return false;
    }
    true
}
