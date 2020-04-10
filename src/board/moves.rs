use crate::board::pieces::Piece;
use crate::board::squares::Square;
use crate::board::Board;
pub enum MoveType {
    NormalMove,
    CastlingKingsSide,
    CastlingQueensSide,
    EnPassant,
}

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
  00 - normal move
  01 - castling king's side
  10 - castling queen's side
  11 - en passant
*/

// There is a lot of info, efficiency is more important than memory in this case.
pub type Move = u32;

macro_rules! append {
    ($num1: expr, $num2: expr, $num2_bit_length: expr) => {
        (($num1) as u32) << ($num2_bit_length) ^ (($num2) as u32)
    };
}

macro_rules! piece_at {
    ($square: expr, $board: expr) => {
        (($board).pieces[$square] as u32)
    };
}

#[macro_export]
macro_rules! new_promotion {
    ($from: expr, $to: expr, $promoted_piece: expr, $board: expr) => {
        append!(
            append!(
                append!(
                    append!(append!($from, $to, 6), piece_at!($to, $board), 4),
                    ($promoted_piece) as u32,
                    4,
                ),
                piece_at!($from, $board),
                4,
            ),
            0,
            2,
        );
    };
}

#[macro_export]
macro_rules! new_move {
    ($from: expr, $to: expr, $board: expr) => {
        new_promotion!($from, $to, piece_at!($from, $board), $board)
    };
}
