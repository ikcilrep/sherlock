extern crate rand;

use crate::board::Board;
use crate::moves::Move;
use crate::pieces::color::Color;
use crate::pieces::pawn;
use crate::pieces::pawn::new_pawn_move;
use rand::rngs::ThreadRng;
use rand::Rng;

impl Board {
    fn generate_random_pawn_capturing_attacker_move(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        rng: &mut ThreadRng,
    ) -> Option<Move> {
        let mut defender_locations = Vec::new();
        self.get_pawns_defending_square_locations(
            king_attacker_location,
            king_location,
            self.state.side,
            &mut defender_locations,
        );

        if defender_locations.is_empty() {
            return None;
        }
        let index = rng.gen_range(0, defender_locations.len());
        let pawn_location = defender_locations[index] as usize;
        if king_attacker_location <= 7 || king_attacker_location >= 56 {
            return Some(pawn::random_promotion(
                pawn_location,
                king_attacker_location,
                self.state.side,
                self,
                rng,
            ));
        }
        return Some(new_pawn_move(
            pawn_location,
            king_attacker_location,
            self.state.side,
            self,
        ));
    }

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

        let should_try_pawn = rng.gen_bool(0.5);
        if should_try_pawn {
            let pawn_move = self.generate_random_pawn_capturing_attacker_move(
                king_location,
                king_attacker_location,
                rng,
            );

            if pawn_move.is_some() {
                return pawn_move;
            }
        }

        let result = self.generate_random_out_of_specific_check_move(
            king_location,
            king_attacker_location,
            8,
            defender_locations_getters,
            rng,
        );

        if result.is_some() {
            result
        } else if !should_try_pawn {
            self.generate_random_pawn_capturing_attacker_move(
                king_location,
                king_attacker_location,
                rng,
            )
        } else {
            None
        }
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
