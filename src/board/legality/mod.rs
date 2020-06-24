use crate::board::Board;
use crate::moves::Move;
use crate::moves::{get_from, get_to};
use crate::pieces::color::{get_piece_color, uncolorize_piece, Color, BLACK};
use crate::pieces::{bishop, king, knight, pawn, queen, rook, BISHOP, EMPTY_SQUARE, KING, KNIGHT};

pub enum GameState {
    Draw,
    Win(Color),
    StillInProgress,
}
pub mod getting_out_of_check;
pub const MOVE_AVAILABILITY_VALIDATORS: [fn(usize, &mut Board) -> bool; 7] = [
    pawn::can_be_moved,
    rook::can_be_moved,
    knight::can_be_moved,
    bishop::can_be_moved,
    queen::can_be_moved,
    king::can_be_moved,
    |_, _| false,
];

impl Board {
    pub fn is_piece_pinned(&mut self, from: i8, to: i8, protected_piece_location: i8) -> bool {
        let protected_piece_file = protected_piece_location & 7;
        let protected_piece_rank = protected_piece_location >> 3;
        let from_file = from & 7;
        let from_rank = from >> 3;
        let to_file = to & 7;
        let to_rank = to >> 3;
        let difference = protected_piece_location - from;
        // I know how hard to read it is, but it's fast.
        (difference >= 0
            && ((from_file == protected_piece_file
                && to_file != from_file
                && self
                    .is_square_attacked_from_south_straight_line_by_slider(from, self.state.side))
                || (from_rank == protected_piece_rank
                    && to_rank != from_rank
                    && self.is_square_attacked_from_west_straight_line_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 9 == 0
                    && self.is_square_attacked_from_northeast_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 7 == 0
                    && self.is_square_attacked_from_northwest_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))))
            || (difference <= 0
                && ((from_file == protected_piece_file
                    && to_file != from_file
                    && self.is_square_attacked_from_north_straight_line_by_slider(
                        from,
                        self.state.side,
                    ))
                    || (from_rank == protected_piece_rank
                        && to_rank != from_rank
                        && self.is_square_attacked_from_east_straight_line_by_slider(
                            from,
                            self.state.side,
                        )))
                || (difference % 9 == 0
                    && self.is_square_attacked_from_southwest_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 7 == 0
                    && self.is_square_attacked_from_southeast_diagonal_by_slider(
                        from,
                        self.state.side,
                    )))
    }

    pub fn is_move_legal(&mut self, half_move: Move) -> bool {
        let king_location = self.state.king_positions[self.state.side as usize];
        let from = get_from(half_move) as i8;

        let to = get_to(half_move) as i8;
        if from == king_location {
            let king = self.state.pieces[king_location as usize];
            self.state.pieces[king_location as usize] = EMPTY_SQUARE;
            let result = self.is_square_attacked(to, self.state.side);
            self.state.pieces[king_location as usize] = king;
            return result;
        }
        return !self.is_piece_pinned(from, to, king_location);
    }

    pub fn is_king_checked(&self, color: Color) -> bool {
        self.is_square_attacked(self.state.king_positions[color as usize], color)
    }

    fn can_any_piece_be_moved(&mut self) -> bool {
        let pieces = self.state.pieces;
        let mut from = 0;
        for piece in pieces.iter() {
            if get_piece_color(*piece) == self.state.side
                && MOVE_AVAILABILITY_VALIDATORS[uncolorize_piece(*piece) as usize](from, self)
            {
                return true;
            }
            from += 1;
        }
        false
    }

    fn is_material_sufficient_to_checkmate(&mut self) -> bool {
        #[inline]
        fn get_square_color(location: usize) -> Color {
            ((location >> 3) & 1 == location & 1) == (location & 1 == 0)
        }

        match self.state.pieces_count {
            2 => false,
            3 => self.state.pieces.iter().any(|piece| {
                let uncolorized_piece = uncolorize_piece(*piece);
                uncolorized_piece != KING
                    && uncolorized_piece != BISHOP
                    && uncolorized_piece != KNIGHT
            }),

            4 => {
                let mut last_bishop_color = BLACK;
                let mut last_square_color = BLACK;
                let mut has_found_bishop = false;
                for (location, piece) in self.state.pieces.iter().enumerate() {
                    let uncolorized_piece = uncolorize_piece(*piece);
                    if uncolorized_piece == BISHOP {
                        let square_color = get_square_color(location);
                        let bishop_color = get_piece_color(*piece);
                        if has_found_bishop {
                            return last_bishop_color == bishop_color
                                || last_square_color != square_color;
                        }

                        last_bishop_color = bishop_color;
                        last_square_color = square_color;
                        has_found_bishop = true;
                    } else if uncolorized_piece != KING {
                        return true;
                    }
                }
                true
            }
            _ => true,
        }
    }

    #[inline]
    fn did_threefold_repetition_occured(&mut self) -> bool {
        self.state.could_be_repeated
            && self
                .states
                .iter()
                .filter(|state| **state == self.state)
                .count()
                >= 3
    }

    #[inline]
    fn is_game_lost(&mut self, king_attackers_locations: &Vec<i8>) -> bool {
        !king_attackers_locations.is_empty()
            && !self.can_get_out_of_check(king_attackers_locations, self.state.side)
    }

    #[inline]
    fn is_game_drawn(&mut self) -> bool {
        self.state.fifty_moves == 100
            || !self.is_material_sufficient_to_checkmate()
            || self.did_threefold_repetition_occured()
            || !self.can_any_piece_be_moved()
    }

    pub fn get_game_state(&mut self, king_attackers_locations: &Vec<i8>) -> GameState {
        // Threefold repetition draw will be implemented in future.
        return if self.is_game_lost(king_attackers_locations) {
            GameState::Win(!self.state.side)
        } else if self.is_game_drawn() {
            GameState::Draw
        } else {
            GameState::StillInProgress
        };
    }
}
