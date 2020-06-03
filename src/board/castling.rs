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
        self.state.has_king_stayed_in_place[color as usize]
            && self.state.has_queens_rook_stayed_in_place[color as usize]
            && self.state.pieces[BISHOP_QUEENS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
            && self.state.pieces[KNIGHTS_QUEENS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
            && self.state.pieces[QUEENS_POSITIONS[color as usize]] == EMPTY_SQUARE
    }

    #[inline]
    pub fn is_castling_kings_side_pseudo_legal(self: &Board, color: Color) -> bool {
        self.state.has_king_stayed_in_place[color as usize]
            && self.state.has_kings_rook_stayed_in_place[color as usize]
            && self.state.pieces[BISHOP_KINGS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
            && self.state.pieces[KNIGHTS_KINGS_SIDE_POSITIONS[color as usize]] == EMPTY_SQUARE
    }

    #[inline]
    pub fn has_castling_kings_side_rights(self: &Board, color: Color) -> bool {
        self.state.has_king_stayed_in_place[color as usize]
            && self.state.has_kings_rook_stayed_in_place[color as usize]
    }

    #[inline]
    pub fn has_castling_queens_side_rights(self: &Board, color: Color) -> bool {
        self.state.has_king_stayed_in_place[color as usize]
            && self.state.has_queens_rook_stayed_in_place[color as usize]
    }
}
