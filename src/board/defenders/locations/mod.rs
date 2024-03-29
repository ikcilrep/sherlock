use crate::board::Board;
use crate::pieces::color::{colorize_piece, uncolorize_piece, Color};
use crate::pieces::pawn::PAWN_STEPS;
use crate::pieces::ColorizedPiece;
use crate::pieces::{king, knight, pawn, BISHOP, EMPTY_SQUARE, KING, KNIGHT, PAWN, QUEEN, ROOK};

pub mod diagonals;
pub mod straight_lines;

impl Board {
    pub fn get_pawns_defending_square_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        let colorized_pawn = colorize_piece(PAWN, defended_color);
        if self.state.pieces[square as usize] == EMPTY_SQUARE {
            let from1 = square - PAWN_STEPS[defended_color as usize][1];
            let from2 = from1 - PAWN_STEPS[defended_color as usize][1];
            if self.is_square_on_board(from1)
                && self.state.pieces[from1 as usize] == colorized_pawn
                && !self.is_piece_pinned(from1, square, defended_piece_location)
            {
                result.push(from1);
            } else if pawn::PAWN_START_RANKS[defended_color as usize] == (from2 >> 3) as usize
                && self.state.pieces[from1 as usize] == EMPTY_SQUARE
                && self.state.pieces[from2 as usize] == colorized_pawn
                && !self.is_piece_pinned(from2, square, defended_piece_location)
            {
                result.push(from2);
            }
        } else {
            let square_file = square & 7;
            let from1 = square - PAWN_STEPS[defended_color as usize][0];
            if self.is_square_on_board(from1)
                && from1 & 7 > square_file
                && self.state.pieces[from1 as usize] == colorized_pawn
                && !self.is_piece_pinned(from1, square, defended_piece_location)
            {
                result.push(from1);
            }

            let from2 = square - PAWN_STEPS[defended_color as usize][2];
            if self.is_square_on_board(from2)
                && from2 & 7 < square_file
                && self.state.pieces[from2 as usize] == colorized_pawn
                && !self.is_piece_pinned(from2, square, defended_piece_location)
            {
                result.push(from2);
            }
        }
    }

    pub fn get_slider_or_queen_defending_square_location(
        &mut self,
        square: i8,
        possible_attacker: ColorizedPiece,
        defended_piece_location: i8,
        defended_color: Color,
        increment: i8,
        predicate: fn(i8, i8) -> bool,
    ) -> Option<i8> {
        let defender_location = self.get_slider_or_queen_attacking_square_location(
            square,
            possible_attacker,
            !defended_color,
            increment,
            predicate,
        );

        return if defender_location.is_some()
            && !self.is_piece_pinned(defender_location.unwrap(), square, defended_piece_location)
        {
            defender_location
        } else {
            None
        };
    }
    pub fn get_slider_defending_square_location(
        &mut self,
        square: i8,
        possible_attacker: ColorizedPiece,
        defended_piece_location: i8,
        increment: i8,
        predicate: fn(i8, i8) -> bool,
    ) -> Option<i8> {
        let defender_location = self.get_slider_attacking_square_location(
            square,
            possible_attacker,
            increment,
            predicate,
        );

        return if defender_location.is_some()
            && !self.is_piece_pinned(defender_location.unwrap(), square, defended_piece_location)
        {
            defender_location
        } else {
            None
        };
    }

    pub fn get_knights_defending_square_locations(
        &mut self,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
        result: &mut Vec<i8>,
    ) {
        let colorized_knight = colorize_piece(KNIGHT, defended_color);

        let square_file = square & 7;
        let moves_to = knight::get_moves_to(square as usize);

        knight::ATTACK_PSEUDO_LEGALITY_VALIDATORS
            .iter()
            .zip(moves_to.iter())
            .filter(|&(is_attack_pseudo_legal, &defender_square)| {
                is_attack_pseudo_legal(square_file, defender_square)
                    && self.state.pieces[defender_square as usize] == colorized_knight
                    && !self.is_piece_pinned(defender_square, square, defended_piece_location)
            })
            .for_each(|(_, &defender_square)| {
                result.push(defender_square);
            });
    }

    pub fn get_pieces_of_type_defending_square_locations(
        &mut self,
        square: i8,
        piece: ColorizedPiece,
    ) -> Vec<i8> {
        let mut result = Vec::new();
        let king_location = self.state.king_positions[self.state.side as usize];
        match uncolorize_piece(piece) {
            KING => {
                if self.can_be_moved(square, self.state.side)
                    && king::get_moves_to(square as usize).contains(&king_location)
                    && !self.is_square_attacked(square, self.state.side)
                {
                    result.push(king_location);
                }
            }
            ROOK => self.get_sliders_defending_square_on_straight_lines_locations(
                square,
                piece,
                king_location,
                &mut result,
            ),
            BISHOP => self.get_sliders_defending_square_on_diagonals_locations(
                square,
                piece,
                king_location,
                &mut result,
            ),
            KNIGHT => self.get_knights_defending_square_locations(
                square,
                king_location,
                self.state.side,
                &mut result,
            ),
            QUEEN => {
                self.get_sliders_defending_square_on_straight_lines_locations(
                    square,
                    piece,
                    king_location,
                    &mut result,
                );

                self.get_sliders_defending_square_on_diagonals_locations(
                    square,
                    piece,
                    king_location,
                    &mut result,
                );
            }
            _ => {}
        };
        result
    }
}
