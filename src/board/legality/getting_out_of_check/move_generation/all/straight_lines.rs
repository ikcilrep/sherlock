use crate::board::Board;
use crate::moves::Move;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::PAWN;

impl Board {
    fn generate_pawn_capturing_attacker_moves(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        result: &mut Vec<Move>,
    ) {
        let mut defender_locations = Vec::new();
        self.get_pawns_defending_square_locations(
            king_attacker_location,
            king_location,
            self.state.side,
            &mut defender_locations,
        );

        let piece = colorize_piece(PAWN, self.state.side);
        for location in defender_locations {
            self.get_moves(location as usize, king_attacker_location, piece, result);
        }
    }

    #[inline]
    pub fn generate_out_of_check_on_file_moves(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        result: &mut Vec<Move>,
    ) {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_rank_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
            Board::get_knights_defending_square_locations,
            |_: &mut Board, _: i8, _: i8, _: Color, _: &mut Vec<i8>| {},
        ];

        self.generate_pawn_capturing_attacker_moves(king_location, king_attacker_location, result);

        self.generate_out_of_specific_check_moves(
            king_location,
            king_attacker_location,
            8,
            defender_locations_getters,
            result,
        );
    }

    #[inline]
    pub fn generate_out_of_check_on_rank_moves(
        &mut self,
        king_location: i8,
        king_attacker_location: i8,
        result: &mut Vec<Move>,
    ) {
        let defender_locations_getters = [
            Board::get_sliders_or_queens_defending_square_on_file_locations,
            Board::get_pieces_defending_square_on_diagonals_locations,
            Board::get_knights_defending_square_locations,
            Board::get_pawns_defending_square_locations,
        ];

        self.generate_out_of_specific_check_moves(
            king_location,
            king_attacker_location,
            1,
            defender_locations_getters,
            result,
        )
    }
}
