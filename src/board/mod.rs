use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE};

pub mod castling;
pub mod constructor;

pub struct Board {
    pub pieces: [ColorizedPiece; 64],
    side: Color,
    fifty_moves: u8,
    has_king_stayed_in_place: [bool; 2],
    has_queens_rook_stayed_in_place: [bool; 2],
    has_kings_rook_stayed_in_place: [bool; 2],
    pub en_passant_square: i8,
}

impl Board {
    #[inline]
    pub fn can_be_moved(self: &Board, to: usize, piece_to_move_color: Color) -> bool {
        self.pieces[to] == EMPTY_SQUARE || get_piece_color(self.pieces[to]) != piece_to_move_color
    }
    #[inline]
    pub fn can_capture(self: &Board, to: usize, piece_to_move_color: Color) -> bool {
        self.pieces[to] != EMPTY_SQUARE && get_piece_color(self.pieces[to]) != piece_to_move_color
    }
}
