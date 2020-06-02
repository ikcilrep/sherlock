use crate::board::board_state::BoardState;
use crate::board::Board;

impl Board {
    pub fn new() -> Board {
        Board {
            state: BoardState::new(),
            last_state: BoardState::new(),
        }
    }
}
