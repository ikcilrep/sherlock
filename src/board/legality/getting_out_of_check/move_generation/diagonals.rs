extern crate rand;

use crate::board::Board;
use crate::moves::constructors::new_move;
use crate::moves::{Move, NULL_MOVE};

use rand::rngs::ThreadRng;
use rand::Rng;

impl Board {
    pub fn generate_random_out_of_check_on_northwest_southeast_diagonal_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Move {
        let (start, min, max) = if king_attacker_location > king_location {
            (
                rng.gen_range(0, king_attacker_location - king_location),
                king_location + 9,
                king_attacker_location,
            )
        } else {
            (
                rng.gen_range(0, king_location - king_attacker_location),
                king_attacker_location,
                king_location - 9,
            )
        };

        let i_upper_limit = max - min + 1;
        let mut i = start;

        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_straight_lines_locations,
            Board::get_pieces_defending_square_on_northeast_southwest_diagonal_locations,
        ];

        let mut defender_locations = Vec::new();

        let indexes = {
            let index1 = rng.gen_range(0, 2) as usize;
            let index2 = (index1 + 1) & 1;
            [index1, index2]
        };

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

            i += 9;
            i %= i_upper_limit;
            i != start
        } {}

        NULL_MOVE
    }
}
