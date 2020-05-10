use crate::board::Board;
use crate::pieces::color::{uncolorize_piece, Color, WHITE};
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE, KING, PAWN};

pub const KING_POSITIONS: [usize; 2] = [4, 60];
pub const KINGS_ROOKS_POSITIONS: [usize; 2] = [7, 63];
pub const QUEENS_ROOKS_POSITIONS: [usize; 2] = [0, 56];
pub const KINGS_ROOKS_AFTER_CASTLING_POSITIONS: [usize; 2] = [5, 61];
pub const QUEENS_ROOKS_AFTER_CASTLING_POSITIONS: [usize; 2] = [3, 59];

pub const INVERSED_PAWN_STEPS: [i8; 2] = [-8, 8];
pub struct BoardState {
    pub side: Color,
    pub fifty_moves: u8,
    pub has_king_stayed_in_place: [bool; 2],
    pub has_queens_rook_stayed_in_place: [bool; 2],
    pub has_kings_rook_stayed_in_place: [bool; 2],
    pub king_positions: [u8; 2],
    pub en_passant_square: i8,
}

impl BoardState {
    pub fn new() -> BoardState {
        BoardState {
            side: WHITE,
            fifty_moves: 0,
            en_passant_square: -1,
            has_king_stayed_in_place: [true, true],
            has_queens_rook_stayed_in_place: [true, true],
            has_kings_rook_stayed_in_place: [true, true],
            king_positions: [4, 60],
        }
    }

    #[inline]
    fn update_fifty_moves(
        self: &mut BoardState,
        last_state: &mut BoardState,
        moved_piece: ColorizedPiece,
        captured_piece: ColorizedPiece,
    ) {
        last_state.fifty_moves = self.fifty_moves;
        if uncolorize_piece(moved_piece) != PAWN && captured_piece == EMPTY_SQUARE {
            self.fifty_moves += 1;
        } else {
            self.fifty_moves = 0;
        }
    }

    #[inline]
    fn update_has_stayed(
        self: &mut BoardState,
        last_state: &mut BoardState,
        from: usize,
        color: usize,
    ) {
        last_state.has_king_stayed_in_place[color] = self.has_king_stayed_in_place[color];
        last_state.has_kings_rook_stayed_in_place[color] =
            self.has_kings_rook_stayed_in_place[color];
        last_state.has_queens_rook_stayed_in_place[color] =
            self.has_queens_rook_stayed_in_place[color];

        self.has_king_stayed_in_place[color] &= KING_POSITIONS[color] != from;
        self.has_kings_rook_stayed_in_place[color] &= KINGS_ROOKS_POSITIONS[color] != from;
        self.has_queens_rook_stayed_in_place[color] &= QUEENS_ROOKS_POSITIONS[color] != from;
    }

    #[inline]
    fn revert_has_stayed(self: &mut BoardState, color: usize) {}

    // Probably, to be optimized.
    #[inline]
    fn update_en_passant_square(
        self: &mut BoardState,
        last_state: &mut BoardState,
        from: usize,
        to: usize,
        moved_piece: ColorizedPiece,
        color: usize,
    ) {
        last_state.en_passant_square = self.en_passant_square;
        self.en_passant_square = if to as i8 - from as i8 == (INVERSED_PAWN_STEPS[!color] << 1)
            && uncolorize_piece(moved_piece) == PAWN
        {
            to as i8 + INVERSED_PAWN_STEPS[color]
        } else {
            -1
        };
    }

    // Probably, to be optimized.
    #[inline]
    fn update_king_position(
        self: &mut BoardState,
        last_state: &mut BoardState,
        moved_piece: ColorizedPiece,
        to: usize,
        color: usize,
    ) {
        if uncolorize_piece(moved_piece) == KING {
            last_state.king_positions[color as usize] = self.king_positions[color as usize];
            self.king_positions[color as usize] = to as u8;
        }
    }

    #[inline]
    fn update_side(self: &mut BoardState, last_state: &mut BoardState) {
        last_state.side = self.side;
        self.side = !self.side;
    }

    #[inline]
    pub fn update(
        self: &mut BoardState,
        last_state: &mut BoardState,
        moved_piece: ColorizedPiece,
        captured_piece: ColorizedPiece,
        from: usize,
        to: usize,
        color: usize,
    ) {
        self.update_has_stayed(last_state, from, color);
        self.update_fifty_moves(last_state, moved_piece, captured_piece);
        self.update_en_passant_square(last_state, from, to, moved_piece, color);
        self.update_king_position(last_state, moved_piece, to, color);
        self.update_side(last_state);
    }

    #[inline]
    pub fn revert(self: &mut BoardState, last_state: &BoardState) {
        let color = last_state.side as usize;
        self.fifty_moves = last_state.fifty_moves;
        self.en_passant_square = last_state.en_passant_square;
        self.king_positions[color] = last_state.king_positions[color];
        self.side = last_state.side;
        self.has_king_stayed_in_place[color] = last_state.has_king_stayed_in_place[color];
        self.has_kings_rook_stayed_in_place[color] =
            last_state.has_kings_rook_stayed_in_place[color];
        self.has_queens_rook_stayed_in_place[color] =
            last_state.has_queens_rook_stayed_in_place[color];
    }
}