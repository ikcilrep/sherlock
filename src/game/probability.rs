extern crate rand;

use rand::rngs::ThreadRng;

use crate::board::legality::GameState;
use crate::board::Board;
use crate::moves::algebraic_notation::to_algebraic_notation;

pub fn play_random_game(board: &mut Board, rng: &mut ThreadRng) -> GameState {
    loop {
        let king_attackers_locations =
            board.get_attackers_of_king_square_locations(board.state.side);
        let state = board.get_game_state(&king_attackers_locations);
        match state {
            GameState::StillInProgress => {}
            _ => return state,
        }

        let half_move = if king_attackers_locations.is_empty() {
            board.generate_random_legal_move(rng)
        } else {
            board
                .generate_random_out_of_check_move(king_attackers_locations, rng)
                .unwrap()
        };

        println!("{}", to_algebraic_notation(half_move, board));
        board.make_move(half_move);
    }
}
