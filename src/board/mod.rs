use crate::moves::{
    get_captured_piece, get_from, get_move_type, get_moved_piece, get_promoted_piece, get_to, Move,
    CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE, EN_PASSANT,
};
use crate::pieces::color::{colorize_piece, get_piece_color, uncolorize_piece, Color};
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE, PAWN, ROOK};

pub mod castling;
pub mod constructor;

pub struct Board {
    pub pieces: [ColorizedPiece; 64],
    side: Color,
    fifty_moves: u8,
    has_king_stayed_in_place: [bool; 2],
    has_queens_rook_stayed_in_place: [bool; 2],
    has_kings_rook_stayed_in_place: [bool; 2],
    last_fifty_moves: u8,
    last_has_king_stayed_in_place: [bool; 2],
    last_has_queens_rook_stayed_in_place: [bool; 2],
    last_has_kings_rook_stayed_in_place: [bool; 2],

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
    fn update_fifty_moves(
        self: &mut Board,
        moved_piece: ColorizedPiece,
        captured_piece: ColorizedPiece,
    ) {
        self.last_fifty_moves = self.fifty_moves;
        if uncolorize_piece(moved_piece) != PAWN && captured_piece == EMPTY_SQUARE {
            self.fifty_moves += 1;
        } else {
            self.fifty_moves = 0;
        }
    }

    #[inline]
    fn update_has_stayed(self: &mut Board, color: usize, from: usize) {
        self.last_has_king_stayed_in_place[color] = self.has_king_stayed_in_place[color];
        self.last_has_kings_rook_stayed_in_place[color] =
            self.has_kings_rook_stayed_in_place[color];
        self.last_has_queens_rook_stayed_in_place[color] =
            self.has_queens_rook_stayed_in_place[color];

        self.has_king_stayed_in_place[color] &= KING_POSITIONS[color] != from;
        self.has_kings_rook_stayed_in_place[color] &= KINGS_ROOKS_POSITIONS[color] != from;
        self.has_queens_rook_stayed_in_place[color] &= QUEENS_ROOKS_POSITIONS[color] != from;
    }

    #[inline]
    fn revert_has_stayed(self: &mut Board, color: usize) {
        self.has_king_stayed_in_place[color] = self.last_has_king_stayed_in_place[color];
        self.has_kings_rook_stayed_in_place[color] =
            self.last_has_kings_rook_stayed_in_place[color];
        self.has_queens_rook_stayed_in_place[color] =
            self.last_has_queens_rook_stayed_in_place[color];
    }

    #[inline]
    pub fn make_move(self: &mut Board, half_move: Move) {
        let from = get_from(half_move);
        let to = get_to(half_move);
        self.pieces[to] = get_promoted_piece(half_move);
        self.pieces[from] = EMPTY_SQUARE;
        let moved_piece = get_moved_piece(half_move);
        let captured_piece = get_captured_piece(half_move);
        let color = get_piece_color(moved_piece) as usize;

        self.update_has_stayed(color, from);
        self.update_fifty_moves(moved_piece, captured_piece);
        self.side = !self.side;

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
    }

    #[inline]
    pub fn undo_move(self: &mut Board, half_move: Move) {
        let moved_piece = get_moved_piece(half_move);
        let color = get_piece_color(moved_piece) as usize;
        let to = get_to(half_move);
        let move_type = get_move_type(half_move);
        if move_type == EN_PASSANT {
            self.pieces[(to as i8 + INVERSED_PAWN_STEPS[color]) as usize] =
                get_captured_piece(half_move);
            self.pieces[to] = EMPTY_SQUARE;
        } else {
            self.pieces[to] = get_captured_piece(half_move);
        }

        self.pieces[get_from(half_move)] = get_moved_piece(half_move);

        self.revert_has_stayed(color);
        self.fifty_moves = self.last_fifty_moves;
        self.side = !self.side;

        match get_move_type(half_move) {
            CASTLING_KINGS_SIDE => {
                self.pieces[KINGS_ROOKS_POSITIONS[color]] =
                    self.pieces[KINGS_ROOKS_AFTER_CASTLING_POSITIONS[color]];
                self.pieces[KINGS_ROOKS_AFTER_CASTLING_POSITIONS[color]] = EMPTY_SQUARE;
            }
            CASTLING_QUEENS_SIDE => {
                self.pieces[QUEENS_ROOKS_POSITIONS[color]] =
                    self.pieces[QUEENS_ROOKS_AFTER_CASTLING_POSITIONS[color]];
                self.pieces[QUEENS_ROOKS_AFTER_CASTLING_POSITIONS[color]] = EMPTY_SQUARE;
            }
            _ => {}
        }
    }
}
