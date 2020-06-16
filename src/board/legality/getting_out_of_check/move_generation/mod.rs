extern crate rand;

use crate::board::Board;
use crate::moves::constructors::new_move;
use crate::moves::{Move, NULL_MOVE};
use crate::pieces::color::Color;
use crate::pieces::king;

use rand::rngs::ThreadRng;
use rand::Rng;

pub mod diagonals;
pub mod straight_lines;

#[inline]
pub fn get_two_random_indexes(rng: &mut ThreadRng) -> [usize; 2] {
    let index1 = rng.gen_range(0, 2) as usize;
    let index2 = (index1 + 1) & 1;
    [index1, index2]
}

#[inline]
pub fn get_start_min_max(
    king_location: i8,
    king_attacker_location: i8,
    increment: i8,
    rng: &mut ThreadRng,
) -> (i8, i8, i8) {
    return if king_attacker_location > king_location {
        (
            rng.gen_range(0, king_attacker_location - king_location),
            king_location + increment,
            king_attacker_location,
        )
    } else {
        (
            rng.gen_range(0, king_location - king_attacker_location),
            king_attacker_location,
            king_location - increment,
        )
    };
}

impl Board {
    #[inline]
    pub fn generate_random_out_of_specific_check_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        increment: i8,
        defender_locations_getters: [fn(&mut Board, i8, i8, Color, &mut Vec<i8>); 2],
        rng: &mut ThreadRng,
    ) -> Move {
        let (start, min, max) =
            get_start_min_max(king_location, king_attacker_location, increment, rng);

        let i_upper_limit = max - min + 1;
        let mut i = start;

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

            i += increment;
            i %= i_upper_limit;
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
                self.generate_random_out_of_check_on_file_move(
                    king_location,
                    attacker_location,
                    rng,
                )
            } else if difference % 9 == 0 {
                self.generate_random_out_of_check_on_northeast_southwest_diagonal_move(
                    king_location,
                    attacker_location,
                    rng,
                )
            } else if difference % 7 == 0 {
                self.generate_random_out_of_check_on_northwest_southeast_diagonal_move(
                    king_location,
                    attacker_location,
                    rng,
                )
            } else {
                NULL_MOVE
            };
        }
        NULL_MOVE
    }
}
