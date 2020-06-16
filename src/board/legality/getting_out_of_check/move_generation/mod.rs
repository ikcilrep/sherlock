extern crate rand;

use crate::board::Board;
use crate::moves::{Move, NULL_MOVE};
use crate::pieces::king;
use rand::rngs::ThreadRng;

pub mod straight_lines;

impl Board {
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
                self.generate_random_out_of_check_on_file_move(
                    king_location,
                    attacker_location,
                    rng,
                )
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
