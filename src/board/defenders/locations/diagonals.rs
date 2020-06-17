use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::BISHOP;

impl Board {
    pub fn get_pieces_defending_square_on_northeast_southwest_diagonal_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        let colorized_bishop = colorize_piece(BISHOP, defended_color);
        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            9,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 > square_file
            },
        ) {
            Some(location) => result.push(location),
            None => {}
        }

        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            -9,
            |attacker_square, square_file| {
                attacker_square >= 0 && attacker_square & 7 < square_file
            },
        ) {
            Some(location) => result.push(location),
            None => {}
        }
    }

    pub fn get_pieces_defending_square_on_northwest_southeast_diagonal_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        let colorized_bishop = colorize_piece(BISHOP, defended_color);
        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            7,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 < square_file
            },
        ) {
            Some(location) => result.push(location),
            None => {}
        }

        match self.get_slider_or_queen_defending_square_location(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            -7,
            |attacker_square, square_file| {
                attacker_square >= 0 && attacker_square & 7 > square_file
            },
        ) {
            Some(location) => result.push(location),
            None => {}
        }
    }
    pub fn get_pieces_defending_square_on_diagonals_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        self.get_pieces_defending_square_on_northwest_southeast_diagonal_locations(
            square,
            defended_piece_location,
            defended_color,
            result,
        );
        self.get_pieces_defending_square_on_northeast_southwest_diagonal_locations(
            square,
            defended_piece_location,
            defended_color,
            result,
        );
    }
}
