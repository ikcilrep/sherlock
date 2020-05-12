use crate::board::attackers;
use crate::board::Board;
use crate::pieces::color::Color;

impl Board {
    fn is_king_checked(self: &Board, color: Color) -> bool {
        self.is_square_attacked(self.state.king_positions[color as usize], color)
    }
}
