use crate::board::Board;
use crate::pieces::color::Color;
use crate::pieces::EMPTY_SQUARE;

const KNIGHTS_KINGS_SIDE_POSITIONS: [usize; 2] = [6, 62];
const KNIGHTS_QUEENS_SIDE_POSITIONS: [usize; 2] = [1, 57];
const BISHOP_KINGS_SIDE_POSITIONS: [usize; 2] = [5, 61];
const BISHOP_QUEENS_SIDE_POSITIONS: [usize; 2] = [2, 58];
const QUEENS_POSITIONS: [usize; 2] = [3, 59];

impl Board {
    #[inline]
    pub fn is_castling_queens_side_pseudo_legal(self: &Board, color: Color) -> bool {
        self.has_king_stayed_in_place[color as usize]
            && self.has_queens_rook_stayed_in_place[color as usize]
            && self.pieces[BISHOP_QUEENS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
            && self.pieces[KNIGHTS_QUEENS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
            && self.pieces[QUEENS_POSITIONS[color as usize]] == EMPTY_SQUARE
    }

    #[inline]
    pub fn is_castling_kings_side_pseudo_legal(self: &Board, color: Color) -> bool {
        self.has_king_stayed_in_place[color as usize]
            && self.has_kings_rook_stayed_in_place[color as usize]
            && self.pieces[BISHOP_KINGS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
            && self.pieces[KNIGHTS_KINGS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
    }
}
