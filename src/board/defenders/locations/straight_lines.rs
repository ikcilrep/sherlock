use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::ROOK;

impl Board {
    pub fn get_sliders_or_queens_defending_square_on_file_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        let colorized_rook = colorize_piece(ROOK, defended_color);
        let location1 = self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            8,
            |defended_square, _| defended_square < 64,
        );

        if location1 != -1 {
            result.push(location1);
        }

        let location2 = self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            -8,
            |defended_square, _| defended_square >= 0,
        );

        if location2 != -1 {
            result.push(location2);
        }
    }

    pub fn get_sliders_or_queens_defending_square_on_rank_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        let colorized_rook = colorize_piece(ROOK, defended_color);
        let location1 = self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            1,
            |defended_square, _| defended_square & 7 != 0,
        );

        if location1 != -1 {
            result.push(location1);
        }

        let location2 = self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            -1,
            |defended_square, _| defended_square & 7 != 7,
        );

        if location2 != -1 {
            result.push(location2);
        }
    }

    pub fn get_sliders_or_queens_defending_square_on_straight_lines_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        self.get_sliders_or_queens_defending_square_on_file_locations(
            square,
            defended_piece_location,
            defended_color,
            result,
        );
        self.get_sliders_or_queens_defending_square_on_rank_locations(
            square,
            defended_piece_location,
            defended_color,
            result,
        );
    }
}
