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
