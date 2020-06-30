use crate::board::Board;
use crate::moves::constructors::new_move;
use crate::moves::Move;
use crate::pieces::color::uncolorize_piece;
use crate::pieces::color::Color;
use crate::pieces::ColorizedPiece;
use crate::pieces::{king, pawn, PAWN};

pub mod diagonals;
pub mod straight_lines;

#[inline]
pub fn get_min_max(king_location: i8, king_attacker_location: i8, increment: i8) -> (i8, i8) {
    return if king_attacker_location > king_location {
        (king_location + increment, king_attacker_location)
    } else {
        (king_attacker_location, king_location - increment)
    };
}
impl Board {
    #[inline]
    pub fn generate_out_of_specific_check_moves(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        increment: i8,
        defender_locations_getters: [fn(&mut Board, i8, i8, Color, &mut Vec<i8>); 4],
        result: &mut Vec<Move>,
    ) {
        let (min, max) = get_min_max(king_location, king_attacker_location, increment);

        for square in (min..=max).step_by(increment as usize) {
            for index in 0..4 {
                let mut locations = Vec::new();
                defender_locations_getters[index](
                    self,
                    square,
                    king_location,
                    self.state.side,
                    &mut locations,
                );

                for location in locations {
                    let piece = self.state.pieces[location as usize];
                    self.get_moves(location as usize, square, piece, result);
                }
            }
        }
    }

    fn get_moves(&self, from: usize, to: i8, piece: ColorizedPiece, result: &mut Vec<Move>) {
        if uncolorize_piece(piece) == PAWN && (to <= 7 || to >= 56) {
            pawn::add_promotions(from, to, self.state.side, self, result)
        } else {
            result.push(new_move(from, to, self))
        }
    }

    fn generate_capturing_attacker_moves(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        result: &mut Vec<Move>,
    ) {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_straight_lines_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
            Board::get_knights_defending_square_locations,
            Board::get_pawns_defending_square_locations,
        ];

        for index in 0..4 {
            let mut defender_locations = Vec::new();
            defender_locations_getters[index](
                self,
                king_attacker_location,
                king_location,
                self.state.side,
                &mut defender_locations,
            );

            for location in defender_locations {
                let piece = self.state.pieces[location as usize];
                self.get_moves(location as usize, king_attacker_location, piece, result);
            }
        }
    }

    pub fn generate_out_of_check_moves(&mut self, king_attackers_locations: &Vec<i8>) -> Vec<Move> {
        let mut result = Vec::new();
        let color = self.state.side;
        let king_location = self.state.king_positions[color as usize];
        let legal_moves_to = king::get_legal_moves_to(king_location as usize, self);
        for to in legal_moves_to {
            result.push(new_move(king_location as usize, to, self));
        }
        if king_attackers_locations.len() == 1 {
            let attacker_location = king_attackers_locations[0];
            let attacker_location_rank = attacker_location >> 3;
            let king_location_rank = king_location >> 3;
            let attacker_location_file = attacker_location & 7;
            let king_location_file = king_location & 7;

            let difference = attacker_location - king_location;

            if attacker_location_rank == king_location_rank {
                self.generate_out_of_check_on_rank_moves(
                    king_location,
                    attacker_location,
                    &mut result,
                );
            } else if attacker_location_file == king_location_file {
                self.generate_out_of_check_on_file_moves(
                    king_location,
                    attacker_location,
                    &mut result,
                );
            } else if difference % 9 == 0 {
                self.generate_out_of_check_on_northeast_southwest_diagonal_moves(
                    king_location,
                    attacker_location,
                    &mut result,
                );
            } else if difference % 7 == 0 {
                self.generate_out_of_check_on_northwest_southeast_diagonal_moves(
                    king_location,
                    attacker_location,
                    &mut result,
                );
            } else {
                self.generate_capturing_attacker_moves(
                    king_location,
                    attacker_location,
                    &mut result,
                );
            };
        }
        result
    }
}
