use crate::board::Board;
use crate::pieces::color::{colorize_piece, Color};
use crate::pieces::ROOK;

impl Board {
    pub fn is_square_defended_from_straight_line_on_file_by_slider(
        self: &Board,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        let colorized_rook = colorize_piece(ROOK, !attacked_color);
        self.is_square_defended_by_slider(
            square,
            colorized_rook,
            attacked_color,
            8,
            |attacker_square, _| attacker_square < 64,
        ) || self.is_square_attacked_by_slider(
            square,
            colorized_rook,
            attacked_color,
            -8,
            |attacker_square, _| attacker_square > 0,
        )
    }

    pub fn is_square_defended_from_straight_loine_on_rank_straight_line_by_slider(
        self: &Board,
        square: i8,
        attacked_color: Color,
    ) -> bool {
        let colorized_rook = colorize_piece(ROOK, !attacked_color);
        self.is_square_defended_by_slider(
            square,
            colorized_rook,
            attacked_color,
            1,
            |attacker_square, _| attacker_square & 7 != 0,
        ) || self.is_square_attacked_by_slider(
            square,
            colorized_rook,
            attacked_color,
            -1,
            |attacker_square, _| attacker_square & 7 != 7,
        )
    }
}
