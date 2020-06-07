use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::{king, knight};
use crate::pieces::{ColorizedPiece, KING, KNIGHT, PAWN};
pub const INVERSED_PAWN_CAPTURES: [[i8; 2]; 2] = [[-7, -9], [9, 7]];

pub mod diagonal;
pub mod locations;
pub mod straight_line;

impl Board {
    pub fn is_square_attacked_by_slider(
        self: &Board,
        square: i8,
        possible_attacker: ColorizedPiece,
        attacked_color: Color,
        increment: i8,
        predicate: fn(i8, i8) -> bool,
    ) -> bool {
        self.get_slider_or_queen_attacking_square_location(
            square,
            possible_attacker,
            attacked_color,
            increment,
            predicate,
        ) > -1
    }

    #[inline]
    pub fn is_square_on_board(self: &Board, square: i8) -> bool {
        square >= 0 && square < 64
    }

    #[inline]
    pub fn is_square_attacked_by_pawn(self: &Board, square: i8, attacked_color: Color) -> bool {
        let colorized_pawn = colorize_piece(PAWN, !attacked_color);
        let square_file = square & 7;
        let attacker_square1 = square + INVERSED_PAWN_CAPTURES[attacked_color as usize][0];
        let attacker_square2 = square + INVERSED_PAWN_CAPTURES[attacked_color as usize][1];
        (self.is_square_on_board(attacker_square1)
            && attacker_square1 & 7 > square_file
            && self.state.pieces[attacker_square1 as usize] == colorized_pawn)
            || (self.is_square_on_board(attacker_square2)
                && attacker_square2 & 7 < square_file
                && self.state.pieces[attacker_square2 as usize] == colorized_pawn)
    }

    fn is_square_attacked_by_knight(self: &Board, square: i8, attacked_color: Color) -> bool {
        let colorized_knight = colorize_piece(KNIGHT, !attacked_color);
        self.is_square_attacked_by_piece(
            square,
            colorized_knight,
            knight::get_moves_to(square as usize),
            knight::MOVE_PSEUDO_LEGALITY_VALIDATORS,
            attacked_color,
        )
    }

    fn is_square_attacked_by_king(self: &Board, square: i8, attacked_color: Color) -> bool {
        let colorized_king = colorize_piece(KING, !attacked_color);
        self.is_square_attacked_by_piece(
            square,
            colorized_king,
            king::get_moves_to(square as usize),
            king::MOVE_PSEUDO_LEGALITY_VALIDATORS,
            attacked_color,
        )
    }

    fn is_square_attacked_by_piece(
        self: &Board,
        square: i8,
        piece: ColorizedPiece,
        moves_to: [i8; 8],
        move_pseudo_legality_validators: [fn(i8, i8, &Board, Color) -> bool; 8],
        attacked_color: Color,
    ) -> bool {
        let square_file = square & 7;

        move_pseudo_legality_validators
            .iter()
            .zip(moves_to.iter())
            .any(|(is_move_pseudo_legal, attacker_square)| {
                is_move_pseudo_legal(square_file, *attacker_square, self, !attacked_color)
                    && self.state.pieces[*attacker_square as usize] == piece
            })
    }

    // Probably, to be optimized.
    pub fn is_square_attacked(self: &Board, square: i8, attacked_color: Color) -> bool {
        self.is_square_attacked_by_king(square, attacked_color)
            || self.is_square_attacked_by_knight(square, attacked_color)
            || self.is_square_attacked_by_pawn(square, attacked_color)
            || self.is_square_attacked_on_straight_line_by_slider(square, attacked_color)
            || self.is_square_attacked_on_diagonal_by_slider(square, attacked_color)
    }
}
