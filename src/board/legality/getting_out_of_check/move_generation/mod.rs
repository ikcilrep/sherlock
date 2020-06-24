extern crate rand;

use crate::board::Board;
use crate::moves::constructors::new_move;
use crate::moves::Move;
use crate::pieces::color::Color;
use crate::pieces::king;

use rand::rngs::ThreadRng;
use rand::Rng;

pub mod diagonals;
pub mod straight_lines;

#[inline]
pub fn get_four_random_indexes(rng: &mut ThreadRng) -> [usize; 4] {
    let index = rng.gen_range(0, 24) as usize;
    [
        [0, 3, 1, 2],
        [0, 2, 3, 1],
        [0, 1, 2, 3],
        [0, 2, 1, 3],
        [0, 1, 3, 2],
        [0, 3, 2, 1],
        [1, 3, 0, 2],
        [1, 2, 3, 0],
        [1, 0, 2, 3],
        [1, 2, 0, 3],
        [1, 0, 3, 2],
        [1, 3, 2, 0],
        [2, 3, 0, 1],
        [2, 1, 3, 0],
        [2, 0, 1, 3],
        [2, 1, 0, 3],
        [2, 0, 3, 1],
        [2, 3, 1, 0],
        [3, 2, 0, 1],
        [3, 1, 2, 0],
        [3, 0, 1, 3],
        [3, 1, 0, 3],
        [3, 0, 3, 1],
        [3, 3, 1, 0],
    ][index]
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
            rng.gen_range(0, (king_attacker_location - king_location) / increment) * increment,
            king_location + increment,
            king_attacker_location,
        )
    } else {
        (
            rng.gen_range(0, (king_location - king_attacker_location) / increment) * increment,
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
        defender_locations_getters: [fn(&mut Board, i8, i8, Color, &mut Vec<i8>); 4],
        rng: &mut ThreadRng,
    ) -> Option<Move> {
        let (start, min, max) =
            get_start_min_max(king_location, king_attacker_location, increment, rng);

        let i_upper_limit = max - min + 1;
        let mut i = start;

        let mut defender_locations = Vec::new();

        let indexes = get_four_random_indexes(rng);

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
                    return Some(new_move(location as usize, square, self));
                }
            }

            i += increment;
            i %= i_upper_limit;
            i != start
        } {}

        None
    }

    fn generate_random_capturing_attacker_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Option<Move> {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_straight_lines_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
            Board::get_knights_defending_square_locations,
            Board::get_pawns_defending_square_locations,
        ];

        let mut defender_locations = Vec::new();
        let indexes = get_four_random_indexes(rng);

        for &index in indexes.iter() {
            defender_locations_getters[index](
                self,
                king_attacker_location,
                king_location,
                self.state.side,
                &mut defender_locations,
            );
            if defender_locations.len() > 0 {
                let location_index = rng.gen_range(0, defender_locations.len());
                let location = defender_locations[location_index];
                return Some(new_move(location as usize, king_attacker_location, self));
            }
        }
        None
    }

    pub fn generate_random_out_of_check_move(
        &mut self,
        king_attackers_locations: &Vec<i8>,
        rng: &mut ThreadRng,
    ) -> Option<Move> {
        let color = self.state.side;
        let king_location = self.state.king_positions[color as usize];
        let legal_moves_to = king::get_legal_moves_to(king_location as usize, self);
        if legal_moves_to.len() > 0 && rng.gen_bool(0.5) {
            return Some(king::generate_random_getting_out_of_check_move(
                king_location as usize,
                &legal_moves_to,
                self,
                rng,
            ));
        }

        if king_attackers_locations.len() == 1 {
            let attacker_location = king_attackers_locations[0];
            let attacker_location_rank = attacker_location >> 3;
            let king_location_rank = king_location >> 3;
            let attacker_location_file = attacker_location & 7;
            let king_location_file = king_location & 7;

            let difference = attacker_location - king_location;

            let half_move = if attacker_location_rank == king_location_rank {
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
                self.generate_random_capturing_attacker_move(king_location, attacker_location, rng)
            };

            if half_move.is_some() {
                return half_move;
            } else if !legal_moves_to.is_empty() {
                return Some(king::generate_random_getting_out_of_check_move(
                    king_location as usize,
                    &legal_moves_to,
                    self,
                    rng,
                ));
            }
        }
        None
    }
}
