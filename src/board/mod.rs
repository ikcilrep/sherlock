use crate::board::state::{
    BoardState, KINGS_ROOKS_AFTER_CASTLING_POSITIONS, KINGS_ROOKS_POSITIONS,
    QUEENS_ROOKS_AFTER_CASTLING_POSITIONS, QUEENS_ROOKS_POSITIONS,
};
use crate::moves::{
    get_captured_piece, get_captured_piece_position, get_from, get_move_type, get_moved_piece,
    get_promoted_piece, get_to, Move, CASTLING_KINGS_SIDE, CASTLING_QUEENS_SIDE,
};
use crate::pieces::color::{get_piece_color, uncolorize_piece, Color};
use crate::pieces::{ColorizedPiece, EMPTY_SQUARE, PAWN};

pub mod attackers;
pub mod castling;
pub mod constructor;
pub mod defenders;
pub mod legality;
pub mod state;

pub struct Board {
    pub state: BoardState,
    pub states: Vec<BoardState>,
}

impl Board {
    #[inline]
    pub fn can_be_moved(&self, to: i8, piece_to_move_color: Color) -> bool {
        self.state.pieces[to as usize] == EMPTY_SQUARE
            || get_piece_color(self.state.pieces[to as usize]) != piece_to_move_color
    }

    #[inline]
    pub fn can_capture(&self, to: i8, piece_to_move_color: Color) -> bool {
        self.state.pieces[to as usize] != EMPTY_SQUARE
            && get_piece_color(self.state.pieces[to as usize]) != piece_to_move_color
    }

    #[inline]
    fn can_position_be_repeated(
        &mut self,
        captured_piece: ColorizedPiece,
        moved_piece: ColorizedPiece,
    ) -> bool {
        captured_piece == EMPTY_SQUARE && uncolorize_piece(moved_piece) != PAWN
    }

    #[inline]
    fn has_the_same_castling_rights(
        &mut self,
        had_castling_kings_side_rights: bool,
        had_castling_queens_side_rights: bool,
        color: Color,
    ) -> bool {
        had_castling_kings_side_rights == self.has_castling_kings_side_rights(color)
            && had_castling_queens_side_rights == self.has_castling_queens_side_rights(color)
    }

    #[inline]
    pub fn make_move(&mut self, half_move: Move) {
        let from = get_from(half_move);
        let to = get_to(half_move);
        let moved_piece = get_moved_piece(half_move);
        let plain_color = get_piece_color(moved_piece);
        let promoted_piece = get_promoted_piece(half_move);
        let color = plain_color as usize;
        let has_castling_queens_side_rights = self.has_castling_queens_side_rights(plain_color);
        let has_castling_kings_side_rights = self.has_castling_kings_side_rights(plain_color);
        self.state.pieces[get_captured_piece_position(half_move)] = EMPTY_SQUARE;

        self.state.pieces[to] = promoted_piece;

        self.state.pieces[from] = EMPTY_SQUARE;

        let captured_piece = get_captured_piece(half_move);

        self.state.pieces_count += (captured_piece != EMPTY_SQUARE) as u8;
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
        self.state
            .update(moved_piece, captured_piece, from, to, color);

        if self.can_position_be_repeated(captured_piece, moved_piece) {
            self.states.push(self.state);
            self.state.could_be_repeated = promoted_piece == moved_piece
                && self.has_the_same_castling_rights(
                    has_castling_kings_side_rights,
                    has_castling_queens_side_rights,
                    plain_color,
                );
        } else {
            self.state.could_be_repeated = false;
        }
    }
}
