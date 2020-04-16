use crate::board::pieces::Piece;

pub type Color = i8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

#[inline]
pub fn get_piece_color(piece: Piece) -> Color {
    piece & 1
}
