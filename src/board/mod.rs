use crate::moves::{
    get_from, get_move_type, get_moved_piece, get_promoted_piece, get_to, Move,
    CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, EN_PASSANT,
};
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

const KING_POSITIONS: [usize; 2] = [4, 60];
const KINGS_ROOKS_POSITIONS: [usize; 2] = [7, 63];
const QUEENS_ROOKS_POSITIONS: [usize; 2] = [0, 56];

const KINGS_ROOKS_AFTER_CASTLING_POSITIONS: [usize; 2] = [5, 61];
const QUEENS_ROOKS_AFTER_CASTLING_POSITIONS: [usize; 2] = [3, 59];
const INVERSED_PAWN_STEPS: [i8; 2] = [-8, 8];

impl Board {
    #[inline]
    pub fn can_be_moved(self: &Board, to: i8, piece_to_move_color: Color) -> bool {
        self.pieces[to as usize] == EMPTY_SQUARE
            || get_piece_color(self.pieces[to as usize]) != piece_to_move_color
    }

    #[inline]
    pub fn can_capture(self: &Board, to: i8, piece_to_move_color: Color) -> bool {
        self.pieces[to as usize] != EMPTY_SQUARE
            && get_piece_color(self.pieces[to as usize]) != piece_to_move_color
    }

    #[inline]
    pub fn make_move(self: &mut Board, half_move: Move) {
        let from = get_from(half_move);
        let to = get_to(half_move);
        self.pieces[to] = get_promoted_piece(half_move);
        self.pieces[from] = EMPTY_SQUARE;
        let color = get_piece_color(get_moved_piece(half_move)) as usize;
        self.has_king_stayed_in_place[color] &= KING_POSITIONS[color] != from;
        self.has_kings_rook_stayed_in_place[color] &= KINGS_ROOKS_POSITIONS[color] != from;
        self.has_queens_rook_stayed_in_place[color] &= QUEENS_ROOKS_POSITIONS[color] != from;

        match get_move_type(half_move) {
            CASTLING_KINGS_SIDE => {
                self.pieces[KINGS_ROOKS_AFTER_CASTLING_POSITIONS[color]] =
                    self.pieces[KINGS_ROOKS_POSITIONS[color]];
                self.pieces[KINGS_ROOKS_POSITIONS[color]] = EMPTY_SQUARE;
            }
            CASTLING_QUEENS_SIDE => {
                self.pieces[QUEENS_ROOKS_AFTER_CASTLING_POSITIONS[color]] =
                    self.pieces[QUEENS_ROOKS_POSITIONS[color]];
                self.pieces[QUEENS_ROOKS_POSITIONS[color]] = EMPTY_SQUARE;
            }
            EN_PASSANT => {
                self.pieces[(to as i8 + INVERSED_PAWN_STEPS[color]) as usize] = EMPTY_SQUARE;
            }
            _ => {}
        }
        self.side = !self.side;
    }
}
