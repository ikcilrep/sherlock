extern crate rand;

use crate::board::Board;
use crate::moves::Move;
use crate::pieces::color::Color;
use rand::rngs::ThreadRng;

impl Board {
    #[inline]
    pub fn generate_random_out_of_check_on_file_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Option<Move> {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_rank_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
            Board::get_knights_defending_square_locations,
            |_: &mut Board, _: i8, _: i8, _: Color, _: &mut Vec<i8>| {},
        ];

        self.generate_random_out_of_specific_check_move(
            king_location,
            king_attacker_location,
            8,
            defender_locations_getters,
            rng,
        )
    }

    #[inline]
    pub fn generate_random_out_of_check_on_rank_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Option<Move> {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_file_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
            Board::get_knights_defending_square_locations,
            Board::get_pawns_defending_square_locations,
        ];

        self.generate_random_out_of_specific_check_move(
            king_location,
            king_attacker_location,
            1,
            defender_locations_getters,
            rng,
        )
    }
}
