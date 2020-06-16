extern crate rand;

use crate::board::Board;
use crate::moves::constructors::new_move;
use crate::moves::{Move, NULL_MOVE};
use crate::pieces::king;
use rand::rngs::ThreadRng;
use rand::Rng;

impl Board {
    fn generate_random_out_of_check_on_rank_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Move {
        let (start, min, max) = if king_attacker_location > king_location {
            (
                rng.gen_range(0, king_attacker_location - king_location),
                king_location + 1,
                king_attacker_location,
            )
        } else {
            (
                rng.gen_range(0, king_location - king_attacker_location),
                king_attacker_location,
                king_location - 1,
            )
        };

        let squares_between_count = max - min;
        let mut i = start;

        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_file_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
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

            defender_locations.clear();
            i += 1;
            i %= squares_between_count;
            i != start
        } {}

        NULL_MOVE
    }

    fn generate_random_out_of_check_move(
        &mut self,
        king_attackers_locations: Vec<i8>,
        rng: &mut ThreadRng,
    ) -> Move {
        let color = self.state.side;
        let king_location = self.state.king_positions[color as usize];
        let legal_moves_to = king::get_legal_moves_to(king_location as usize, self);
        if legal_moves_to.len() > 0 {
            return king::generate_random_getting_of_check_move(
                king_location as usize,
                &legal_moves_to,
                self,
                rng,
            );
        }

        if king_attackers_locations.len() == 1 {
            let attacker_location = king_attackers_locations[0];
            let attacker_location_rank = attacker_location >> 3;
            let king_location_rank = king_location >> 3;
            let attacker_location_file = attacker_location & 7;
            let king_location_file = king_location & 7;

            let difference = attacker_location - king_location;

            return if attacker_location_rank == king_location_rank {
                self.generate_random_out_of_check_on_rank_move(
                    king_location,
                    attacker_location,
                    rng,
                )
            } else if attacker_location_file == king_location_file {
                NULL_MOVE
            } else if difference % 9 == 0 {
                NULL_MOVE
            } else if difference % 7 == 0 {
                NULL_MOVE
            } else {
                NULL_MOVE
            };
        }
        NULL_MOVE
    }
}
