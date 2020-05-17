use crate::board::attackers;
use crate::board::Board;
use crate::pieces::color::{colorize_piece, get_piece_color, uncolorize_piece, Color};
use crate::pieces::{knight, ColorizedPiece, BISHOP, EMPTY_SQUARE, KING, KNIGHT, QUEEN, ROOK};

impl Board {
    #[inline]
    fn is_square_not_occupied_by_color(self: &Board, square: usize, color: Color) -> bool {
        self.pieces[square] == EMPTY_SQUARE || get_piece_color(self.pieces[square]) != color
    }

    fn get_slider_attacking_square_location(
        self: &Board,
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
            if self.pieces[attacker_square as usize] == possible_attacker {
                return attacker_square;
            }
            attacker_square += increment;
        }
        -1
    }

    pub fn get_slider_or_queen_attacking_square_location(
        self: &Board,
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
            let attacker = self.pieces[attacker_square as usize];
            if attacker == possible_attacker || attacker == colorized_queen {
                return attacker_square;
            }
            attacker_square += increment;
        }
        -1
    }

    fn get_piece_attacking_square_on_straight_lines_locations(
        self: &Board,
        piece: ColorizedPiece,
        square: i8,
        attacked_color: Color,
    ) -> Vec<i8> {
        let mut result = Vec::new();
        let location1 = self.get_slider_attacking_square_location(
            square,
            piece,
            attacked_color,
            8,
            |attacker_square, _| attacker_square < 64,
        );

        if location1 != -1 {
            result.push(location1);
        }

        let location2 = self.get_slider_attacking_square_location(
            square,
            piece,
            attacked_color,
            -8,
            |attacker_square, _| attacker_square > 0,
        );

        if location2 != -1 {
            result.push(location2)
        }

        let location3 = self.get_slider_attacking_square_location(
            square,
            piece,
            attacked_color,
            1,
            |attacker_square, _| attacker_square & 7 != 0,
        );

        if location3 != -1 {
            result.push(location3);
        }

        let location4 = self.get_slider_attacking_square_location(
            square,
            piece,
            attacked_color,
            -1,
            |attacker_square, _| attacker_square & 7 != 7,
        );

        if location4 != -1 {
            result.push(location4);
        }

        result
    }

    pub fn get_piece_attacking_square_on_diagonals_locations(
        self: &Board,
        piece: ColorizedPiece,
        square: i8,
        attacked_color: Color,
    ) -> Vec<i8> {
        let mut result = Vec::new();

        let location1 = self.get_slider_attacking_square_location(
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

        let location2 = self.get_slider_attacking_square_location(
            square,
            piece,
            attacked_color,
            -9,
            |attacker_square, square_file| attacker_square > 0 && attacker_square & 7 < square_file,
        );

        if location2 != -1 {
            result.push(location2);
        }

        let location3 = self.get_slider_attacking_square_location(
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

        let location4 = self.get_slider_attacking_square_location(
            square,
            piece,
            attacked_color,
            -7,
            |attacker_square, square_file| attacker_square > 0 && attacker_square & 7 > square_file,
        );

        if location4 != -1 {
            result.push(location4);
        }

        result
    }

    fn get_pieces_attacking_square_locations(
        self: &Board,
        piece: ColorizedPiece,
        square: i8,
        moves_to: [i8; 8],
        move_pseudo_legality_validators: [fn(i8, i8, &Board, Color) -> bool; 8],
        attacked_color: Color,
    ) -> Vec<i8> {
        let square_file = square & 7;

        move_pseudo_legality_validators
            .iter()
            .zip(moves_to.iter())
            .filter(|(is_move_pseudo_legal, attacker_square)| {
                is_move_pseudo_legal(square_file, **attacker_square, self, !attacked_color)
                    && self.pieces[**attacker_square as usize] == piece
            })
            .map(|(_, attacker_square)| *attacker_square)
            .collect()
    }

    pub fn get_pieces_of_type_attacking_square_locations(
        self: &Board,
        square: i8,
        piece: ColorizedPiece,
        attacked_color: Color,
    ) -> Vec<i8> {
        match uncolorize_piece(piece) {
            KNIGHT => self.get_pieces_attacking_square_locations(
                piece,
                square,
                knight::get_moves_to(square as usize),
                knight::MOVE_PSEUDO_LEGALITY_VALIDATORS,
                attacked_color,
            ),
            KING => self.get_pieces_attacking_square_locations(
                piece,
                square,
                knight::get_moves_to(square as usize),
                knight::MOVE_PSEUDO_LEGALITY_VALIDATORS,
                attacked_color,
            ),

            ROOK => self.get_piece_attacking_square_on_straight_lines_locations(
                piece,
                square,
                attacked_color,
            ),
            BISHOP => self.get_piece_attacking_square_on_diagonals_locations(
                piece,
                square,
                attacked_color,
            ),

            QUEEN => {
                let mut result = self.get_piece_attacking_square_on_straight_lines_locations(
                    piece,
                    square,
                    attacked_color,
                );
                result.append(&mut self.get_piece_attacking_square_on_diagonals_locations(
                    piece,
                    square,
                    attacked_color,
                ));
                result
            }
            _ => panic!("Invalid piece type."),
        }
    }
}
