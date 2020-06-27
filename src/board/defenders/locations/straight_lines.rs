use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::{ColorizedPiece, ROOK};

impl Board {
    pub fn get_sliders_or_queens_defending_square_on_file_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        let colorized_rook = colorize_piece(ROOK, defended_color);
        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            8,
            |defended_square, _| defended_square < 64,
        ) {
            Some(location) => result.push(location),
            None => {}
        }

        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            -8,
            |defended_square, _| defended_square >= 0,
        ) {
            Some(location) => result.push(location),
            None => {}
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
        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            1,
            |defended_square, _| defended_square & 7 != 0,
        ) {
            Some(location) => result.push(location),
            None => {}
        }

        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_rook,
            defended_piece_location,
            defended_color,
            -1,
            |defended_square, _| defended_square & 7 != 7,
        ) {
            Some(location) => result.push(location),
            None => {}
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

    pub fn get_sliders_defending_square_on_straight_lines_locations(
        &mut self,
        square: i8,
        slider: ColorizedPiece,
        defended_piece_location: i8,
        result: &mut Vec<i8>,
    ) {
        match self.get_slider_defending_square_location(
            square,
            slider,
            defended_piece_location,
            1,
            |defended_square, _| defended_square & 7 != 0,
        ) {
            Some(location) => result.push(location),
            None => {}
        }

        match self.get_slider_defending_square_location(
            square,
            slider,
            defended_piece_location,
            -1,
            |defended_square, _| defended_square & 7 != 7,
        ) {
            Some(location) => result.push(location),
            None => {}
        }
        match self.get_slider_defending_square_location(
            square,
            slider,
            defended_piece_location,
            8,
            |defended_square, _| defended_square < 64,
        ) {
            Some(location) => result.push(location),
            None => {}
        }

        match self.get_slider_defending_square_location(
            square,
            slider,
            defended_piece_location,
            -8,
            |defended_square, _| defended_square >= 0,
        ) {
            Some(location) => result.push(location),
            None => {}
        }
    }
}
