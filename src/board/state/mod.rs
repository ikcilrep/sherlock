use crate::pieces::color::{uncolorize_piece, Color};
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE, KING, PAWN};

pub const KING_POSITIONS: [usize; 2] = [4, 60];
pub const KINGS_ROOKS_POSITIONS: [usize; 2] = [7, 63];
pub const QUEENS_ROOKS_POSITIONS: [usize; 2] = [0, 56];
pub const KINGS_ROOKS_AFTER_CASTLING_POSITIONS: [usize; 2] = [5, 61];
pub const QUEENS_ROOKS_AFTER_CASTLING_POSITIONS: [usize; 2] = [3, 59];

pub const INVERSED_PAWN_STEPS: [i8; 2] = [-8, 8];

pub mod constructor;

#[derive(Copy, Clone)]
pub struct BoardState {
    pub pieces_count: u8,
    pub pieces: [ColorizedPiece; 64],
    pub side: Color,
    pub fifty_moves: u8,
    pub has_king_stayed_in_place: [bool; 2],
    pub has_queens_rook_stayed_in_place: [bool; 2],
    pub has_kings_rook_stayed_in_place: [bool; 2],
    pub king_positions: [i8; 2],
    pub en_passant_square: i8,
    pub could_be_repeated: bool,
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.side == other.side
            && self.pieces_count == other.pieces_count
            && self.en_passant_square == other.en_passant_square
            && self.fifty_moves == other.fifty_moves
            && self.has_king_stayed_in_place == other.has_king_stayed_in_place
            && self.has_queens_rook_stayed_in_place == other.has_queens_rook_stayed_in_place
            && self.has_kings_rook_stayed_in_place == other.has_kings_rook_stayed_in_place
            && self.king_positions == other.king_positions
            && self
                .pieces
                .iter()
                .enumerate()
                .all(|(index, piece)| other.pieces[index] == *piece)
    }
}

impl Eq for BoardState {}

impl BoardState {
    #[inline]
    fn update_fifty_moves(&mut self, moved_piece: ColorizedPiece, captured_piece: ColorizedPiece) {
        if uncolorize_piece(moved_piece) != PAWN && captured_piece == EMPTY_SQUARE {
            self.fifty_moves += 1;
        } else {
            self.fifty_moves = 0;
        }
    }

    #[inline]
    fn update_has_stayed(&mut self, from: usize, color: usize) {
        self.has_king_stayed_in_place[color] &= KING_POSITIONS[color] != from;
        self.has_kings_rook_stayed_in_place[color] &= KINGS_ROOKS_POSITIONS[color] != from;
        self.has_queens_rook_stayed_in_place[color] &= QUEENS_ROOKS_POSITIONS[color] != from;
    }

    // Probably, to be optimized.
    #[inline]
    fn update_en_passant_square(
        &mut self,
        from: usize,
        to: usize,
        moved_piece: ColorizedPiece,
        color: usize,
    ) {
        self.en_passant_square = if to as i8 - from as i8 == (-INVERSED_PAWN_STEPS[color] << 1)
            && uncolorize_piece(moved_piece) == PAWN
        {
            to as i8 + INVERSED_PAWN_STEPS[color]
        } else {
            -1
        };
    }

    // Probably, to be optimized.
    #[inline]
    fn update_king_position(&mut self, moved_piece: ColorizedPiece, to: usize, color: usize) {
        if uncolorize_piece(moved_piece) == KING {
            self.king_positions[color as usize] = to as i8;
        }
    }

    #[inline]
    fn update_side(&mut self) {
        self.side = !self.side;
    }

    #[inline]
    pub fn update(
        &mut self,
        moved_piece: ColorizedPiece,
        captured_piece: ColorizedPiece,
        from: usize,
        to: usize,
        color: usize,
    ) {
        self.update_has_stayed(from, color);
        self.update_fifty_moves(moved_piece, captured_piece);
        self.update_en_passant_square(from, to, moved_piece, color);
        self.update_king_position(moved_piece, to, color);
        self.update_side();
    }

    #[inline]
    pub fn revert(&mut self, last_state: &BoardState) {
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
