use crate::pieces::{ColorizedPiece, Piece};

pub type Color = u8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

#[inline]
pub fn get_piece_color(piece: ColorizedPiece) -> Color {
    piece & 1
}

#[inline]
pub fn colorize_piece(piece: Piece, color: Color) -> ColorizedPiece {
    piece << 1 ^ color
}
