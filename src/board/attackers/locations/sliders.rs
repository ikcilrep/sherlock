use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::{ColorizedPiece, QUEEN};

type AttackingSliderFinder = fn(&Board, i8, ColorizedPiece, Color, i8, fn(i8, i8) -> bool) -> i8;

impl Board {
    pub fn get_slider_attacking_square_location(
        &self,
        square: i8,
        possible_attacker: ColorizedPiece,
        attacked_color: Color,
        increment: i8,
        predicate: fn(i8, i8) -> bool,
    ) -> i8 {
        let mut attacker_square = square + increment;
        let square_file = square & 7;
        while predicate(attacker_square, square_file)
            && self.is_square_not_occupied_by_color(attacker_square as usize, attacked_color)
        {
            if self.state.pieces[attacker_square as usize] == possible_attacker {
                return attacker_square;
            }
            attacker_square += increment;
        }
        -1
    }

    pub fn get_slider_or_queen_attacking_square_location(
        &self,
        square: i8,
        possible_attacker: ColorizedPiece,
        attacked_color: Color,
        increment: i8,
        predicate: fn(i8, i8) -> bool,
    ) -> i8 {
        let colorized_queen = colorize_piece(QUEEN, !attacked_color);
        let mut attacker_square = square + increment;
        let square_file = square & 7;
        while predicate(attacker_square, square_file)
            && self.is_square_not_occupied_by_color(attacker_square as usize, attacked_color)
        {
            let attacker = self.state.pieces[attacker_square as usize];
            if attacker == possible_attacker || attacker == colorized_queen {
                return attacker_square;
            }
            attacker_square += increment;
        }
        -1
    }

    pub fn get_pieces_attacking_square_on_straight_lines_locations(
        &self,
        piece: ColorizedPiece,
        square: i8,
        attacked_color: Color,
        get_slider_attacking_square_location: AttackingSliderFinder,
        result: &mut Vec<i8>,
    ) {
        let location1 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            8,
            |attacker_square, _| attacker_square < 64,
        );

        if location1 != -1 {
            result.push(location1);
        }

        let location2 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            -8,
            |attacker_square, _| attacker_square >= 0,
        );

        if location2 != -1 {
            result.push(location2)
        }

        let location3 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            1,
            |attacker_square, _| attacker_square & 7 != 0,
        );

        if location3 != -1 {
            result.push(location3);
        }

        let location4 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            -1,
            |attacker_square, _| attacker_square & 7 != 7,
        );

        if location4 != -1 {
            result.push(location4);
        }
    }

    pub fn get_pieces_attacking_square_on_diagonals_locations(
        &self,
        piece: ColorizedPiece,
        square: i8,
        attacked_color: Color,
        get_slider_attacking_square_location: AttackingSliderFinder,
        result: &mut Vec<i8>,
    ) {
        let location1 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            9,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 > square_file
            },
        );

        if location1 != -1 {
            result.push(location1);
        }

        let location2 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            -9,
            |attacker_square, square_file| {
                attacker_square >= 0 && attacker_square & 7 < square_file
            },
        );

        if location2 != -1 {
            result.push(location2);
        }

        let location3 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            7,
            |attacker_square, square_file| {
                attacker_square < 64 && attacker_square & 7 < square_file
            },
        );

        if location3 != -1 {
            result.push(location3);
        }

        let location4 = get_slider_attacking_square_location(
            self,
            square,
            piece,
            attacked_color,
            -7,
            |attacker_square, square_file| {
                attacker_square >= 0 && attacker_square & 7 > square_file
            },
        );

        if location4 != -1 {
            result.push(location4);
        }
    }
}
