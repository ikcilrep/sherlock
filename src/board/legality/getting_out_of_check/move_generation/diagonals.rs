extern crate rand;

use crate::board::Board;
use crate::moves::Move;

use rand::rngs::ThreadRng;

impl Board {
    #[inline]
    pub fn generate_random_out_of_check_on_northeast_southwest_diagonal_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Move {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_straight_lines_locations,
            Board::get_pieces_defending_square_on_northwest_southeast_diagonal_locations,
            Board::get_knights_defending_square_locations,
        ];

        self.generate_random_out_of_specific_check_move(
            king_location,
            king_attacker_location,
            9,
            defender_locations_getters,
            rng,
        )
    }

    #[inline]
    pub fn generate_random_out_of_check_on_northwest_southeast_diagonal_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Move {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_straight_lines_locations,
            Board::get_pieces_defending_square_on_northeast_southwest_diagonal_locations,
            Board::get_knights_defending_square_locations,
        ];

        self.generate_random_out_of_specific_check_move(
            king_location,
            king_attacker_location,
            7,
            defender_locations_getters,
            rng,
        )
    }
}
