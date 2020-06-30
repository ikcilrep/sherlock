use crate::board::Board;
use crate::moves::Move;

impl Board {
    #[inline]
    pub fn generate_out_of_check_on_northeast_southwest_diagonal_moves(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        result: &mut Vec<Move>,
    ) {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_straight_lines_locations,
            Board::get_pieces_defending_square_on_northwest_southeast_diagonal_locations,
            Board::get_knights_defending_square_locations,
            Board::get_pawns_defending_square_locations,
        ];

        self.generate_out_of_specific_check_moves(
            king_location,
            king_attacker_location,
            9,
            defender_locations_getters,
            result,
        )
    }

    #[inline]
    pub fn generate_out_of_check_on_northwest_southeast_diagonal_moves(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        result: &mut Vec<Move>,
    ) {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_straight_lines_locations,
            Board::get_pieces_defending_square_on_northeast_southwest_diagonal_locations,
            Board::get_knights_defending_square_locations,
            Board::get_pawns_defending_square_locations,
        ];

        self.generate_out_of_specific_check_moves(
            king_location,
            king_attacker_location,
            7,
            defender_locations_getters,
            result,
        )
    }
}
