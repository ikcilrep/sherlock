use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::BISHOP;

impl Board {
    pub fn is_square_defended_from_northeast_southwest_diagonal_by_slider(
        self: &mut Board,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
    ) -> bool {
        let colorized_bishop = colorize_piece(BISHOP, defended_color);
        self.is_square_defended_by_slider(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            9,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 > square_file
            },
        ) || self.is_square_defended_by_slider(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            -9,
            |attacker_square, square_file| attacker_square > 0 && attacker_square & 7 < square_file,
        )
    }

    pub fn is_square_defended_from_northwest_southeast_diagonal_by_slider(
        self: &mut Board,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
    ) -> bool {
        let colorized_bishop = colorize_piece(BISHOP, defended_color);
        self.is_square_defended_by_slider(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            7,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 < square_file
            },
        ) || self.is_square_defended_by_slider(
            square,
            colorized_bishop,
            defended_piece_location,
            defended_color,
            -7,
            |attacker_square, square_file| attacker_square > 0 && attacker_square & 7 > square_file,
        )
    }

    pub fn is_square_defended_from_diagonal_by_slider(
        self: &mut Board,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
    ) -> bool {
        self.is_square_defended_from_northeast_southwest_diagonal_by_slider(
            square,
            defended_piece_location,
            defended_color,
        ) || self.is_square_defended_from_northwest_southeast_diagonal_by_slider(
            square,
            defended_piece_location,
            defended_color,
        )
    }
}
