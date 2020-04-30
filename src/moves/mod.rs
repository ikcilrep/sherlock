use crate::pieces::ColorizedPiece;

// Excluding 0 padding.
// First is most significant.
/* bits 1-6 - from
  bits 7-12 - to
  bits 13-16 - captured piece
  bits 17-20 - promoted piece
  bits 21-24 - moved piece
  bits 25-30 - captured piece position
  bits 31-32 - info:
  00 - castling king's side
  01 - castling queen's side
  10 - en passant
  11 - normal move
*/

// There is a lot of info, efficiency is more important than memory in this case.
pub mod constructors;
pub type Move = u32;
pub type MoveType = u8;
pub const CASTLING_KINGS_SIDE: MoveType = 0;
pub const CASTLING_QUEENS_SIDE: MoveType = 1;
pub const EN_PASSANT: MoveType = 2;
pub const NORMAL_MOVE: MoveType = 3;

#[inline]
pub fn get_from(half_move: Move) -> usize {
    (half_move >> 26) as usize
}

#[inline]
pub fn get_to(half_move: Move) -> usize {
    ((half_move >> 20) & 63) as usize
}

#[inline]
pub fn get_captured_piece_position(half_move: Move) -> usize {
    ((half_move >> 2) & 63) as usize
}

#[inline]
pub fn get_captured_piece(half_move: Move) -> ColorizedPiece {
    ((half_move >> 16) & 15) as ColorizedPiece
}

#[inline]
pub fn get_promoted_piece(half_move: Move) -> ColorizedPiece {
    ((half_move >> 12) & 15) as ColorizedPiece
}

#[inline]
pub fn get_moved_piece(half_move: Move) -> ColorizedPiece {
    ((half_move >> 8) & 15) as ColorizedPiece
}

#[inline]
pub fn get_move_type(half_move: Move) -> MoveType {
    (half_move & 3) as MoveType
}
