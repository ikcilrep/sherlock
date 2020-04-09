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

// All pieces are without color (without last bit).
/* bits 1-6 - from
  bits 7-12 - to
  bits 13-15 - captured piece
  bits 16-18 - promoted piece
  bits 18-20 - moved piece
  bit  21 - color of moved piece
  bits 21-23 - info:
  00 - normal move
  01 - castling king's side
  10 - castling queen's side
  11 - en passant
*/

// There is a lot of info, efficiency is more important than memory in this case.
pub type Move = u32;
