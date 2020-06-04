use crate::board::state::BoardState;
use crate::board::Board;

impl Board {
    pub fn new() -> Board {
        let state = BoardState::new();
        let states = vec![state];
        Board {
            state: state,
            states: states,
        }
    }
}
