use crate::pieces::{ColorizedPiece, Piece};

pub type Color = bool;
pub const WHITE: Color = false;
pub const BLACK: Color = true;

#[inline]
pub fn get_piece_color(piece: ColorizedPiece) -> Color {
    (piece & 1) != 0
}

#[inline]
pub fn colorize_piece(piece: Piece, color: Color) -> ColorizedPiece {
    piece << 1 ^ (color as u8)
}

#[inline]
pub fn uncolorize_piece(piece: ColorizedPiece) -> Piece {
    piece >> 1
}
