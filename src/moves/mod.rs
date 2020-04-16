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
pub mod constructors;
pub type Move = u32;
pub type MoveType = u32;
pub const CASTLING_KINGS_SIDE: MoveType = 0;
pub const CASTLING_QUEENS_SIDE: MoveType = 1;
pub const EN_PASSANT: MoveType = 2;
pub const NORMAL_MOVE: MoveType = 3;
