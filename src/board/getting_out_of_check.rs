use crate::board::Board;
use crate::pieces::color::Color;
use crate::pieces::{king, pawn};

impl Board {
    fn can_get_out_of_check_on_rank(
        self: &mut Board,
        attacker_location: i8,
        king_location: i8,
        color: Color,
    ) -> bool {
        fn is_square_defended_by_not_pawn(
            square: i8,
            defended_piece_location: i8,
            defended_color: Color,
            board: &mut Board,
        ) -> bool {
            board.is_square_defended_by_knight(square, defended_piece_location, defended_color)
                || board.is_square_defended_from_diagonal_by_slider(
                    square,
                    defended_piece_location,
                    defended_color,
                )
                || board.is_square_defended_from_straight_line_on_file_by_slider(
                    square,
                    defended_piece_location,
                    defended_color,
                )
        }

        fn is_defended(square: i8, king_location: i8, color: Color, board: &mut Board) -> bool {
            pawn::can_be_moved_on_empty_square_without_capture(square, board)
                || is_square_defended_by_not_pawn(square, king_location, color, board)
        }
        pawn::can_capture_on_enemy_occupied_square(attacker_location, self)
            || is_square_defended_by_not_pawn(attacker_location, king_location, color, self)
            || (attacker_location > king_location
                && ((king_location + 1)..attacker_location)
                    .any(|square| is_defended(square, king_location, color, self)))
            || (attacker_location..king_location)
                .any(|square| is_defended(square, king_location, color, self))
    }

    fn can_get_out_of_check_on_file(
        self: &mut Board,
        attacker_location: i8,
        king_location: i8,
        color: Color,
    ) -> bool {
        fn is_defended(
            square: i8,
            defended_piece_location: i8,
            defended_color: Color,
            board: &mut Board,
        ) -> bool {
            board.is_square_defended_by_knight(square, defended_piece_location, defended_color)
                || board.is_square_defended_from_diagonal_by_slider(
                    square,
                    defended_piece_location,
                    defended_color,
                )
                || board.is_square_defended_from_straight_line_on_rank_by_slider(
                    square,
                    defended_piece_location,
                    defended_color,
                )
        }

        pawn::can_capture_on_enemy_occupied_square(attacker_location, self)
            || is_defended(attacker_location, king_location, color, self)
            || (attacker_location > king_location
                && ((king_location + 8)..attacker_location)
                    .step_by(8)
                    .any(|square| is_defended(square, king_location, color, self))
                || (attacker_location..king_location)
                    .step_by(8)
                    .any(|square| is_defended(square, king_location, color, self)))
    }

    fn can_get_out_of_check_on_northwest_southeast_diagonal(
        self: &mut Board,
        attacker_location: i8,
        king_location: i8,
        color: Color,
    ) -> bool {
        fn is_square_defended_by_not_pawn(
            square: i8,
            defended_piece_location: i8,
            defended_color: Color,
            board: &mut Board,
        ) -> bool {
            board.is_square_defended_by_knight(square, defended_piece_location, defended_color)
                || board.is_square_defended_from_northeast_southwest_diagonal_by_slider(
                    square,
                    defended_piece_location,
                    defended_color,
                )
                || board.is_square_defended_from_straight_line_by_slider(
                    square,
                    defended_piece_location,
                    defended_color,
                )
        }

        fn is_defended(square: i8, king_location: i8, color: Color, board: &mut Board) -> bool {
            pawn::can_be_moved_on_empty_square_without_capture(square, board)
                || is_square_defended_by_not_pawn(square, king_location, color, board)
        }

        pawn::can_capture_on_enemy_occupied_square(attacker_location, self)
            || is_square_defended_by_not_pawn(attacker_location, king_location, color, self)
            || (attacker_location > king_location
                && ((king_location + 7)..attacker_location)
                    .step_by(7)
                    .any(|square| is_defended(square, king_location, color, self))
                || (attacker_location..king_location)
                    .step_by(7)
                    .any(|square| is_defended(square, king_location, color, self)))
    }

    pub fn can_get_out_of_check(
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
                let attacker_location_file = attacker_location & 7;
                let king_location_file = king_location & 7;

                return if attacker_location_rank == king_location_rank {
                    self.can_get_out_of_check_on_rank(attacker_location, king_location, color)
                } else if attacker_location_file == king_location_file {
                    self.can_get_out_of_check_on_file(attacker_location, king_location, color)
                } else if (attacker_location - king_location) % 7 == 0 {
                    self.can_get_out_of_check_on_northwest_southeast_diagonal(
                        attacker_location,
                        king_location,
                        color,
                    )
                } else {
                    false
                };
                // diagonals and knight in future
            }
            _ => king::can_be_moved(king_location as usize, self),
        };
    }
}
