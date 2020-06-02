use crate::board::board_state::{
    BoardState, KINGS_ROOKS_AFTER_CASTLING_POSITIONS, KINGS_ROOKS_POSITIONS,
    QUEENS_ROOKS_AFTER_CASTLING_POSITIONS, QUEENS_ROOKS_POSITIONS,
};
use crate::moves::{
    get_captured_piece, get_captured_piece_position, get_from, get_move_type, get_moved_piece,
    get_promoted_piece, get_to, Move, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE,
};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::EMPTY_SQUARE;

pub mod attackers;
pub mod attackers_locations;
pub mod board_state;
pub mod board_state_constructor;
pub mod castling;
pub mod constructor;
pub mod defenders;
pub mod diagonal_attackers;
pub mod diagonal_defenders;
pub mod getting_out_of_check;
pub mod legality;
pub mod straight_line_attackers;
pub mod straight_line_defenders;

pub struct Board {
    pub state: BoardState,
}

impl Board {
    #[inline]
    pub fn can_be_moved(self: &Board, to: i8, piece_to_move_color: Color) -> bool {
        self.state.pieces[to as usize] == EMPTY_SQUARE
            || get_piece_color(self.state.pieces[to as usize]) != piece_to_move_color
    }

    #[inline]
    pub fn can_capture(self: &Board, to: i8, piece_to_move_color: Color) -> bool {
        self.state.pieces[to as usize] != EMPTY_SQUARE
            && get_piece_color(self.state.pieces[to as usize]) != piece_to_move_color
    }

    #[inline]
    pub fn make_move(self: &mut Board, half_move: Move) {
        let from = get_from(half_move);
        let to = get_to(half_move);
        self.state.pieces[get_captured_piece_position(half_move)] = EMPTY_SQUARE;
        self.state.pieces[to] = get_promoted_piece(half_move);
        self.state.pieces[from] = EMPTY_SQUARE;
        let moved_piece = get_moved_piece(half_move);
        let captured_piece = get_captured_piece(half_move);
        self.state.pieces_count += (captured_piece != EMPTY_SQUARE) as u8;
        let color = get_piece_color(moved_piece) as usize;
        self.state
            .update(moved_piece, captured_piece, from, to, color);

        match get_move_type(half_move) {
            CASTLING_KINGS_SIDE => {
                self.state.pieces[KINGS_ROOKS_AFTER_CASTLING_POSITIONS[color]] =
                    self.state.pieces[KINGS_ROOKS_POSITIONS[color]];
                self.state.pieces[KINGS_ROOKS_POSITIONS[color]] = EMPTY_SQUARE;
            }
            CASTLING_QUEENS_SIDE => {
                self.state.pieces[QUEENS_ROOKS_AFTER_CASTLING_POSITIONS[color]] =
                    self.state.pieces[QUEENS_ROOKS_POSITIONS[color]];
                self.state.pieces[QUEENS_ROOKS_POSITIONS[color]] = EMPTY_SQUARE;
            }
            _ => {}
        }
    }
}
