use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::BISHOP;

impl Board {
    pub fn is_square_attacked_from_northeast_diagonal_by_slider(
        &self,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        let colorized_bishop = colorize_piece(BISHOP, !attacked_color);
        self.is_square_attacked_by_slider(
            square,
            colorized_bishop,
            attacked_color,
            9,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 > square_file
            },
        )
    }

    pub fn is_square_attacked_from_southwest_diagonal_by_slider(
        &self,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        let colorized_bishop = colorize_piece(BISHOP, !attacked_color);
        self.is_square_attacked_by_slider(
            square,
            colorized_bishop,
            attacked_color,
            -9,
            |attacker_square, square_file| {
                attacker_square >= 0 && attacker_square & 7 < square_file
            },
        )
    }

    pub fn is_square_attacked_from_northwest_diagonal_by_slider(
        &self,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        let colorized_bishop = colorize_piece(BISHOP, !attacked_color);
        self.is_square_attacked_by_slider(
            square,
            colorized_bishop,
            attacked_color,
            7,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 < square_file
            },
        )
    }

    pub fn is_square_attacked_from_southeast_diagonal_by_slider(
        &self,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        let colorized_bishop = colorize_piece(BISHOP, !attacked_color);
        self.is_square_attacked_by_slider(
            square,
            colorized_bishop,
            attacked_color,
            -7,
            |attacker_square, square_file| {
                attacker_square >= 0 && attacker_square & 7 > square_file
            },
        )
    }

    pub fn is_square_attacked_on_diagonal_by_slider(
        &self,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        self.is_square_attacked_from_northeast_diagonal_by_slider(square, attacked_color)
            || self.is_square_attacked_from_southwest_diagonal_by_slider(square, attacked_color)
            || self.is_square_attacked_from_northwest_diagonal_by_slider(square, attacked_color)
            || self.is_square_attacked_from_southeast_diagonal_by_slider(square, attacked_color)
    }
}
