use crate::board::board_state::{
    BoardState, KINGS_ROOKS_AFTER_CASTLING_POSITIONS, KINGS_ROOKS_POSITIONS,
    QUEENS_ROOKS_AFTER_CASTLING_POSITIONS, QUEENS_ROOKS_POSITIONS,
};
use crate::moves::{
    get_captured_piece, get_captured_piece_position, get_from, get_move_type, get_moved_piece,
    get_promoted_piece, get_to, Move, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE,
};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE};

pub mod attackers;
pub mod board_state;
pub mod castling;
pub mod checks;
pub mod constructor;

pub struct Board {
    pub pieces: [ColorizedPiece; 64],
    pub state: BoardState,
    last_state: BoardState,
}

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
        self.pieces[get_captured_piece_position(half_move)] = EMPTY_SQUARE;
        self.pieces[to] = get_promoted_piece(half_move);
        self.pieces[from] = EMPTY_SQUARE;
        let moved_piece = get_moved_piece(half_move);
        let captured_piece = get_captured_piece(half_move);
        let color = get_piece_color(moved_piece) as usize;
        self.state.update(
            &mut self.last_state,
            moved_piece,
            captured_piece,
            from,
            to,
            color,
        );

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
            _ => {}
        }
    }

    #[inline]
    pub fn undo_move(self: &mut Board, half_move: Move) {
        let moved_piece = get_moved_piece(half_move);
        let color = get_piece_color(moved_piece) as usize;
        let to = get_to(half_move);

        self.pieces[get_from(half_move)] = self.pieces[to];
        self.pieces[to] = EMPTY_SQUARE;
        self.pieces[get_captured_piece_position(half_move)] = get_captured_piece(half_move);

        self.state.revert(&self.last_state);

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
