extern crate rand;

use crate::board::legality::getting_out_of_check::move_generation::{
    get_start_min_max, get_two_random_indexes,
};
use crate::board::Board;
use crate::moves::constructors::new_move;
use crate::moves::{Move, NULL_MOVE};
use rand::rngs::ThreadRng;
use rand::Rng;

impl Board {
    pub fn generate_random_out_of_check_on_file_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Move {
        let (start, min, max) = get_start_min_max(king_location, king_attacker_location, 8, rng);

        let i_upper_limit = max - min + 1;
        let mut i = start;

        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_rank_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
        ];

        let mut defender_locations = Vec::new();

        let indexes = get_two_random_indexes(rng);

        while {
            let square = min + i;
            for &index in indexes.iter() {
                defender_locations_getters[index](
                    self,
                    square,
                    king_location,
                    self.state.side,
                    &mut defender_locations,
                );
                if defender_locations.len() > 0 {
                    let location_index = rng.gen_range(0, defender_locations.len());
                    let location = defender_locations[location_index];
                    return new_move(location as usize, square, self);
                }
            }

            i += 8;
            i %= i_upper_limit;
            i != start
        } {}

        NULL_MOVE
    }

    pub fn generate_random_out_of_check_on_rank_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Move {
        let (start, min, max) = get_start_min_max(king_location, king_attacker_location, 1, rng);

        let i_upper_limit = max - min + 1;
        let mut i = start;

        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_file_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
        ];

        let mut defender_locations = Vec::new();

        let indexes = get_two_random_indexes(rng);

        while {
            let square = min + i;
            for &index in indexes.iter() {
                defender_locations_getters[index](
                    self,
                    square,
                    king_location,
                    self.state.side,
                    &mut defender_locations,
                );
                if defender_locations.len() > 0 {
                    let location_index = rng.gen_range(0, defender_locations.len());
                    let location = defender_locations[location_index];
                    return new_move(location as usize, square, self);
                }
            }

            i += 1;
            i %= i_upper_limit;
            i != start
        } {}

        NULL_MOVE
    }
}
