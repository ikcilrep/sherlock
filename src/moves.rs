use crate::board::Board;
use crate::pieces::color::Color;
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE};
/* pub enum CaptureType {
    NoCapture,
    PawnCapture,
    RookCapture,
    KnightCapture,
    BishopCapture,
    QueenCapture,
}*/

// Excluding 0 padding.
// First is most significant.
/* bits 1-6 - from
  bits 7-12 - to
  bits 13-16 - captured piece
  bits 17-20 - promoted piece
  bits 21-24 - moved piece
  bits 24-26 - info:
  00 - castling king's side
  01 - castling queen's side
  10 - en passant
  11 - normal move
*/

// There is a lot of info, efficiency is more important than memory in this case.
pub type Move = u32;
pub type MoveType = u32;
pub const KING_TO_POSITIONS: [[i32; 2]; 2] = [[6, 62], [2, 56]];
pub const CASTLING_KINGS_SIDE: MoveType = 0;
pub const CASTLING_QUEENS_SIDE: MoveType = 1;
pub const EN_PASSANT: MoveType = 2;
pub const NORMAL_MOVE: MoveType = 3;

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
                append!(append!(from, to, 6), board.pieces[to as usize], 4),
                promoted_piece,
                4
            ),
            board.pieces[from],
            4
        ),
        NORMAL_MOVE,
        2
    )
}

#[inline]
pub fn new_move(from: usize, to: i8, board: &Board) -> Move {
    new_promotion(from, to, board.pieces[from], board)
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
        castling_type,
        2
    )
}
