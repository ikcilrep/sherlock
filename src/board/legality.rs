use crate::board::Board;
use crate::moves::Move;
use crate::pieces::color::Color;

impl Board {
    pub fn is_move_legal(self: &mut Board, half_move: Move) -> bool {
        self.make_move(half_move);
        if self.is_king_checked(!self.state.side) {
            self.undo_move(half_move);
            return false;
        }
        true
    }

    fn is_king_checked(self: &Board, color: Color) -> bool {
        self.is_square_attacked(self.state.king_positions[color as usize], color)
    }
}
