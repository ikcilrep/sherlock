use crate::board::Board;
use crate::moves::Move;
use crate::moves::{get_from, get_to};
use crate::pieces::color::{get_piece_color, Color};
use crate::pieces::{bishop, king, knight, pawn, queen, rook, EMPTY_SQUARE};

enum GameState {
    Draw,
    Win(Color),
    StillInProgress,
}

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
    pub fn is_piece_pinned(
        self: &mut Board,
        from: i8,
        to: i8,
        protected_piece_location: i8,
    ) -> bool {
        let protected_piece_file = protected_piece_location & 7;
        let protected_piece_rank = protected_piece_location >> 3;
        let from_file = from & 7;
        let from_rank = from >> 3;
        let to_file = to & 7;
        let to_rank = to >> 3;
        let difference = protected_piece_location - from;
        // I know how hard to read it is, but it's fast.
        (difference > 0
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
                || (difference % 7 == 0
                    && self.is_square_attacked_from_northwest_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 9 == 0
                    && self.is_square_attacked_from_northeast_diagonal_by_slider(
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
                || (difference % 7 == 0
                    && self.is_square_attacked_from_southeast_diagonal_by_slider(
                        from,
                        self.state.side,
                    ))
                || (difference % 9 == 0
                    && self.is_square_attacked_from_southwest_diagonal_by_slider(
                        from,
                        self.state.side,
                    )))
    }

    pub fn is_move_legal(self: &mut Board, half_move: Move) -> bool {
        let king_location = self.state.king_positions[self.state.side as usize];
        let from = get_from(half_move) as i8;

        let to = get_to(half_move) as i8;
        if from == king_location {
            let king = self.pieces[king_location as usize];
            self.pieces[king_location as usize] = EMPTY_SQUARE;

            let result = self.is_square_attacked(to, self.state.side);
            self.pieces[king_location as usize] = king;
            return result;
        }
        return !self.is_piece_pinned(from, to, king_location);
    }

    fn is_king_checked(self: &Board, color: Color) -> bool {
        self.is_square_attacked(self.state.king_positions[color as usize], color)
    }

    fn can_any_piece_be_moved(self: &mut Board) -> bool {
        let pieces = self.pieces;
        let mut from = 0;
        for piece in pieces.iter() {
            if get_piece_color(*piece) == self.state.side
                && MOVE_AVAILABILITY_VALIDATORS[*piece as usize](from, self)
            {
                return true;
            }
            from += 1;
        }
        false
    }

    /*fn is_square_defended_not_from_rank_by_not_pawn(
        self: &mut Board,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
    ) -> bool {
        self.is_square_defended_by_knight(square, defended_piece_location, defended_color)
            || self.is_square_defended_from_diagonal_by_slider(
                square,
                defended_piece_location,
                defended_color,
            )
            || self.is_square_defended_from_straight_line_on_file_by_slider(
                square,
                defended_piece_location,
                defended_color,
            )
    }*/

    /*fn can_get_out_of_check(
        self: &mut Board,
        king_attackers_locations: &Vec<i8>,
        color: Color,
    ) -> bool {
        let king_location = self.state.king_positions[color as usize];
        return match king_attackers_locations.len() {
            1 => {
                if king::can_be_moved(king_location as usize, self) {
                    return true;
                }
                let attacker_location = king_attackers_locations[0];
                let attacker_location_rank = attacker_location >> 3;
                let king_location_rank = king_location >> 3;

                if attacker_location_rank == king_location_rank {
                    if attacker_location > king_location {
                        return (king_location..attacker_location).any(|square| {
                            pawn::can_be_moved_on_empty_square_without_capture(square, self)
                                || self.is_square_defended_not_from_rank_by_not_pawn(
                                    square,
                                    king_location,
                                    color,
                                )
                        });
                    }
                }
                false
            }
            _ => king::can_be_moved(king_location as usize, self),
        };
    }*/

    fn get_game_result(self: &mut Board) -> GameState {
        // Temporary solution !self.can_any_piece_be_moved will be replaced with more customized function if king is checked.
        // Threefold repetition draw will be implemented in future.
        return if !self.can_any_piece_be_moved() {
            if self.is_king_checked(self.state.side) {
                GameState::Win(!self.state.side)
            } else {
                GameState::Draw
            }
        } else if self.state.fifty_moves == 100 {
            GameState::Draw
        } else {
            GameState::StillInProgress
        };
    }
}
