use crate::board::Board;
use crate::pieces::color::{colorize_piece, get_piece_color, Color};
use crate::pieces::pawn::PAWN_STEPS;
use crate::pieces::{knight, ColorizedPiece, BISHOP, EMPTY_SQUARE, KNIGHT, PAWN, ROOK};

pub mod sliders;

impl Board {
    #[inline]
    fn is_square_not_occupied_by_color(&self, square: usize, color: Color) -> bool {
        self.state.pieces[square] == EMPTY_SQUARE
            || get_piece_color(self.state.pieces[square]) != color
    }

    pub fn get_attackers_of_king_square_locations(&self, attacked_color: Color) -> Vec<i8> {
        let square = self.state.king_positions[attacked_color as usize];
        let mut result = Vec::new();
        self.get_pieces_attacking_square_locations(
            colorize_piece(KNIGHT, !attacked_color),
            square,
            knight::get_moves_to(square as usize),
            knight::ATTACK_PSEUDO_LEGALITY_VALIDATORS,
            &mut result,
        );
        self.get_pieces_attacking_square_on_straight_lines_locations(
            colorize_piece(ROOK, !attacked_color),
            square,
            attacked_color,
            Board::get_slider_or_queen_attacking_square_location,
            &mut result,
        );

        self.get_pawn_attacking_king_square_location(square, attacked_color, &mut result);

        self.get_pieces_attacking_square_on_diagonals_locations(
            colorize_piece(BISHOP, !attacked_color),
            square,
            attacked_color,
            Board::get_slider_or_queen_attacking_square_location,
            &mut result,
        );

        result
    }

    fn get_pawn_attacking_king_square_location(
        &self,
        king_square: i8,
        king_color: Color,
        result: &mut Vec<i8>,
    ) {
        let king_square_file = king_square & 7;
        let color = king_color as usize;
        let from1 = king_square + PAWN_STEPS[color][0];
        let from2 = king_square + PAWN_STEPS[color][2];
        let colorized_pawn = colorize_piece(PAWN, !king_color);
        if self.is_square_on_board(from1)
            && king_square_file > from1 & 7
            && self.state.pieces[from1 as usize] == colorized_pawn
        {
            result.push(from1);
        } else if self.is_square_on_board(from2)
            && king_square_file < from2 & 7
            && self.state.pieces[from2 as usize] == colorized_pawn
        {
            result.push(from2);
        }
    }

    fn get_pieces_attacking_square_locations(
        &self,
        piece: ColorizedPiece,
        square: i8,
        moves_to: [i8; 8],
        attack_pseudo_legality_validators: [fn(i8, i8) -> bool; 8],
        result: &mut Vec<i8>,
    ) {
        let square_file = square & 7;

        attack_pseudo_legality_validators
            .iter()
            .zip(moves_to.iter())
            .filter(|(is_attack_pseudo_legal, attacker_square)| {
                is_attack_pseudo_legal(square_file, **attacker_square)
                    && self.state.pieces[**attacker_square as usize] == piece
            })
            .for_each(|(_, attacker_square)| result.push(*attacker_square))
    }
}
