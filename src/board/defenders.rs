use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::knight;
use crate::pieces::{ColorizedPiece, KNIGHT};

impl Board {
    pub fn is_square_defended_by_slider(
        self: &mut Board,
        square: i8,
        possible_attacker: ColorizedPiece,
        defended_piece_location: i8,
        defended_color: Color,
        increment: i8,
        predicate: fn(i8, i8) -> bool,
    ) -> bool {
        let defender_location = self.get_slider_or_queen_attacking_square_location(
            square,
            possible_attacker,
            !defended_color,
            increment,
            predicate,
        );
        defender_location > -1
            && !self.is_piece_pinned(defender_location, square, defended_piece_location)
    }

    pub fn is_square_defended_by_knight(
        self: &mut Board,
        square: i8,
        defended_piece_location: i8,
        defended_color: Color,
    ) -> bool {
        let colorized_knight = colorize_piece(KNIGHT, defended_color);

        let square_file = square & 7;
        let moves_to = knight::get_moves_to(square as usize);

        knight::MOVE_PSEUDO_LEGALITY_VALIDATORS
            .iter()
            .zip(moves_to.iter())
            .any(|(is_move_pseudo_legal, defender_square)| {
                is_move_pseudo_legal(square_file, *defender_square, self, defended_color)
                    && self.state.pieces[*defender_square as usize] == colorized_knight
                    && !self.is_piece_pinned(*defender_square, square, defended_piece_location)
            })
    }
}
