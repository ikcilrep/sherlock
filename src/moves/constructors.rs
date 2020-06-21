use crate::board::Board;
use crate::moves::{Move, MoveType, EN_PASSANT, NORMAL_MOVE};
use crate::pieces::color::Color;
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE};

pub const KING_TO_POSITIONS: [[i32; 2]; 2] = [[6, 62], [2, 56]];
const INVERSED_PAWN_STEPS: [i8; 2] = [-8, 8];

#[macro_export]
macro_rules! append {
    ($num1: expr, $num2: expr, $num2_bit_length: expr) => {
        ((($num1) as u32) << ($num2_bit_length) ^ (($num2) as u32))
    };
}

#[inline]
pub fn new_promotion(from: usize, to: i8, promoted_piece: ColorizedPiece, board: &Board) -> Move {
    append!(
        append!(
            append!(
                append!(
                    append!(append!(from, to, 6), board.state.pieces[to as usize], 4),
                    promoted_piece,
                    4
                ),
                board.state.pieces[from],
                4
            ),
            to,
            6
        ),
        NORMAL_MOVE,
        2
    )
}

#[inline]
pub fn new_en_passant(from: usize, to: i8, pawn_color: Color,  board: &Board) -> Move {
    let captured_piece_location = (to + INVERSED_PAWN_STEPS[pawn_color as usize]) as usize;
    append!(
        append!(
            append!(
                append!(
                    append!(
                        append!(from, to, 6),
                        board.state.pieces[captured_piece_location],
                        4
                    ),
                    board.state.pieces[from],
                    4
                ),
                board.state.pieces[from],
                4
            ),
            captured_piece_location,
            6
        ),
        EN_PASSANT,
        2
    )
}

#[inline]
pub fn new_move(from: usize, to: i8, board: &Board) -> Move {
    new_promotion(from, to, board.state.pieces[from], board)
}

#[inline]
pub fn new_castling(
    castling_type: MoveType,
    from: usize,
    king: ColorizedPiece,
    king_color: Color,
) -> Move {
    append!(
        append!(
            append!(
                append!(
                    append!(
                        append!(
                            from,
                            KING_TO_POSITIONS[(castling_type) as usize][(king_color) as usize],
                            6
                        ),
                        EMPTY_SQUARE,
                        4
                    ),
                    king,
                    4
                ),
                king,
                4
            ),
            KING_TO_POSITIONS[(castling_type) as usize][(king_color) as usize],
            6
        ),
        castling_type,
        2
    )
}
