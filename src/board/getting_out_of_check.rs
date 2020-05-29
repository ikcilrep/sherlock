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
        fn is_defended(square: i8, king_location: i8, color: Color, board: &mut Board) -> bool {
            pawn::can_be_moved_on_empty_square_without_capture(square, board)
                || board.is_square_defended_not_from_rank_by_not_pawn(square, king_location, color)
        }

        pawn::can_capture_on_enemy_occupied_square(attacker_location, self)
            || self.is_square_defended_not_from_rank_by_not_pawn(
                attacker_location,
                king_location,
                color,
            )
            || (attacker_location > king_location
                && (king_location..attacker_location)
                    .any(|square| is_defended(square, king_location, color, self)))
            || (attacker_location..king_location)
                .any(|square| is_defended(square, king_location, color, self))
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

                if attacker_location_rank == king_location_rank {
                    return self.can_get_out_of_check_on_rank(
                        attacker_location,
                        king_location,
                        color,
                    );
                } // diagonals, else in future
                false
            }
            _ => king::can_be_moved(king_location as usize, self),
        };
    }
}
